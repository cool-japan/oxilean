# OxiLean Verify — Engineering Brief

**Project:** `oxilean-verify` — an independent Lean 4 proof checker
**Demo:** *Kernel in a Tab*
**Owner:** COOLJAPAN OU (Team Kitasan)
**License:** Apache-2.0
**Status:** Draft v1 — for engineering execution

---

## 0. The prize

> **A second kernel. Written by someone else. In a different language. That agrees.**

Lean 4 is the fastest-growing proof assistant in mathematics, and Mathlib is the largest formal library ever built. Every theorem in it is certified by **one** kernel — Lean's own. That kernel is excellent, and everyone in the field knows the argument is circular anyway. **The de Bruijn criterion says the trusted computing base should be small, and independently checkable.** In practice, "independently" means: by a kernel that was not written by the people who wrote the prover.

The field takes this seriously. `trepplein` (Scala) and `lean4lean` (Mario Carneiro) exist for exactly this reason. But:

- **There is no maintained, independent, Pure Rust, memory-safe kernel checking Lean 4 exports.**
- **We have one.** 115,444 SLoC, 3,444 tests, **zero external crate dependencies**, no `unsafe`.

That combination — *independent*, *memory-safe*, *zero-dependency* — is not a marketing line for a TCB. **It is the entire specification of what a TCB is supposed to be**, and it is a property Lean's own C++-backed kernel cannot claim.

So the project is this: point our kernel at Lean's own exported proofs, check them, publish the coverage, and then **do it in a browser tab.**

```
Drop Mathlib.export into a web page.
Watch an independent kernel verify it. Live. No server.
```

This is small, bounded, technically serious, and it lands in a community that will *want* it to exist. It is also the highest-prestige thing in the entire COOLJAPAN portfolio, and — see §11 — the one that produces a number a stranger can check in ten minutes.

---

## 1. What we are standing on

This is not a from-scratch project. It is a **short, sharp application of assets that already exist and are already good.**

| Asset | State | Why it matters here |
|---|---|---|
| **`oxilean-kernel`** — CiC type checker | 115,444 SLoC · 3,444 tests · **zero external deps** · no `unsafe` | **The crown jewel.** This project exists to point it at something worthy. |
| **`oxilean-parse`** — Lean-syntax parser | 62,293 SLoC · **181,890 Mathlib4 declarations · 99.7% parse compatibility** across 7,759 files | Proof that this team handles **Mathlib-scale input** and does not flinch. Not on this project's critical path (see §3.1) — which is *good news*, because it means the verifier ships fast. |
| `oxilean-elab` / `oxilean-meta` | elaborator, unification, typeclass synthesis, tactics | The ITP product. A separate, longer game (§10). |
| Mathlib end-to-end | **320 theorems** verified through parse → elaborate → tactic execution | Real, and honestly labelled as a distinct track. Keep it that way. |

**The kernel is the asset. This brief is about spending it well.**

---

## 2. Why `lean4export` is exactly the right interface

Lean 4 ships a tool that dumps a fully-elaborated environment in a **flat, line-based, index-deduplicated, documented format**. Names, universe levels, expressions, and declarations, each in its own numbered table, referenced by index.

What is *not* in that file:

- ❌ notation
- ❌ macros
- ❌ tactics
- ❌ elaboration
- ❌ implicit arguments, instance resolution, unification

**Just kernel terms.** `Expr` with `bvar`, `sort`, `const`, `app`, `lam`, `forallE`, `letE`, `proj`, and literals. Universe levels with `zero`, `succ`, `max`, `imax`, `param`. Declarations: definitions, theorems, axioms, inductives, recursors, quotients.

This is a **fixed, boring, stable interface** — and boring is precisely what you want between two independent implementations. It is why `trepplein` and `lean4lean` both consume it.

**The reader is a week of work.** The *checking* is the project. That is the right ratio: almost all of the effort goes into the part that is actually interesting and actually hard.

Work from the format specification in the `lean4export` repository, not from memory. Pin the exact `lean4export` commit and the exact Lean toolchain version in `rust-toolchain`-style config, and record both in every report.

---

## 3. Architecture

```
   Lean 4 (upstream, unmodified — we change nothing on their side)
        │   lake exe lean4export Mathlib.Analysis.SpecialFunctions.Log.Basic
        ▼
   ┌────────────────────────────┐
   │  .export                    │   flat · indexed · no notation · no macros
   │  (fixed, documented)        │   the whole point: nothing to disagree about
   └──────────────┬──────────────┘
                  │
   ┌──────────────▼──────────────┐
   │  oxilean-export     [NEW]   │   ≤ 3,000 SLoC · zero deps · no unsafe · fuzzed
   │  reader → Expr / Level /    │   ~1 week
   │  Declaration                │
   └──────────────┬──────────────┘
                  │  kernel terms
   ┌──────────────▼──────────────┐
   │  oxilean-kernel  [EXISTING] │   ← the asset. Extend it (§4). Do not dilute it.
   │  TCB · 0 deps · no unsafe   │
   └──────────────┬──────────────┘
                  │
        ┌─────────┴──────────┐
        ▼                    ▼
   oxilean-verify       @cooljapan/oxilean-verify
   (native CLI)         (WASM — the demo, §6)
```

### 3.1 What is deliberately *not* in this build

`oxilean-parse`, `oxilean-elab`, `oxilean-meta`, `oxilean-std`, `oxilean-codegen`, `oxilean-runtime`, `oxilean-build`, `oxilean-lint`, `oxilean-cli`.

Not because they are bad — they are the ITP, and the ITP is the long game. But **the verifier's entire value is that its TCB is small and auditable.** A reviewer must be able to read everything that stands between a `.export` file and a verdict. That is the product.

Enforce with a dependency allow-list gate (§8.3). This is also why the WASM will be small.

---

## 4. The hard parts — i.e. the actual engineering

Your kernel's stated feature list is the right list: universe hierarchy, dependent types, inductive types, proof irrelevance, universe polymorphism. Real Lean 4 exports need more. **These five are the project.**

### 4.1 Quotient types

`Quot`, `Quot.mk`, `Quot.lift`, `Quot.ind`, plus the computation rule
`Quot.lift f h (Quot.mk r a) ≡ f a`.

Quotients are a **kernel primitive** in Lean, not a definition — and Mathlib is built on them (`ZMod`, `Cardinal`, quotient groups, `Setoid` everywhere). Export files contain `#QUOT` declarations. **Check whether the kernel has these first**; nothing meaningful in Mathlib verifies without them.

### 4.2 Definitional eta for structures

Lean's kernel treats `s` and `⟨s.1, s.2⟩` as definitionally equal for single-constructor non-recursive inductives. Without it, a large class of real proofs simply fail to typecheck, in ways that look like mysterious `isDefEq` failures rather than like a missing feature. **Implement it deliberately, and test it deliberately.**

### 4.3 Literal reduction — and a zero-dependency bignum

Lean's kernel special-cases `Nat` and `String` literals: `Nat.add`, `Nat.mul`, `Nat.sub`, `Nat.div`, `Nat.mod`, `Nat.pow`, `Nat.gcd`, `Nat.beq`, `Nat.ble`, and the bitwise operations are computed **directly on the literal**, with GMP underneath, rather than unfolded through the unary/binary representation.

Without this, `decide`-heavy and `norm_num`-heavy proofs do not merely run slowly — they do not terminate in any useful sense.

**And here is the interesting constraint: the kernel has zero external dependencies. So we write our own arbitrary-precision `Nat`.**

That is not a burden; it is a genuinely satisfying, self-contained piece of work with hard correctness criteria and a clear performance target:

- schoolbook + Karatsuba, a few hundred lines
- exhaustive differential test against a reference implementation *outside* the TCB (a test-only dev-dependency is fine — it never ships)
- fuzz it
- **it must stay small enough to audit by eye**, because it is inside the trusted base

A hand-written, audited, `unsafe`-free bignum inside a zero-dependency TCB is, on its own, something you can point at with a straight face.

### 4.4 Recursors for nested and mutual inductives

Iota reduction, generated recursors, motive/minor-premise construction, and — crucially — **re-deriving the recursors ourselves rather than trusting the exported ones.** Trusting Lean's recursors would defeat the entire purpose. Same for **strict positivity** and the universe constraints on inductive declarations: an independent checker re-checks them.

This is the part where "independent" stops being a word and starts being work.

### 4.5 Universe level definitional equality

`max` / `imax` normalisation and `isDefEq` on levels is one of the genuinely subtle corners of any CiC kernel, and it is where independent implementations most often diverge from each other. Expect to spend real time here. Expect it to be interesting.

---

## 5. Differential testing — where the fun is

Two other independent checkers exist: **`lean4lean`** (Lean 4 kernel written in Lean 4) and **`trepplein`** (Scala, Lean 3-era, but instructive).

Build a differential harness. For every declaration in the corpus, compare our verdict against theirs.

**A disagreement is not a bug report. It is a finding.** Three possible outcomes, and every one of them is worth having:

1. **We're wrong.** → We fix a real bug in our kernel. Good.
2. **They're wrong.** → We've found a bug in a checker the community relies on. Very good.
3. **We're both right, and the spec is ambiguous.** → That is a paper.

This is why verification people find this work enjoyable rather than grim: **there is no losing branch.** Set the harness up early (M1), not late — the disagreements are the most informative signal you will get, and you want them arriving while the design is still soft.

---

## 6. The demo: *Kernel in a Tab*

Static page. GitHub Pages. No backend, no account, no `SharedArrayBuffer`, no COOP/COEP. Must run under `python3 -m http.server` — checked in CI.

```
0:00  A page. One drop zone.
      Badge:  kernel: 287 KB wasm · 0 external dependencies · 0 unsafe · 0 bytes uploaded

0:02  User drops  Mathlib.Analysis.SpecialFunctions.Log.Basic.export

0:03  Declarations stream in, checking live, one line each:

        ✓  Real.log_le_sub_one_of_pos              1.2 ms
        ✓  Real.add_pow_le_pow_mul_pow_of_sq_le_sq 4.8 ms
        ✓  Real.exp_log                            0.9 ms
        ⊘  Real.exp_approx        unsupported: Nat literal reduction (§4.3)
        ✓  Real.log_nonneg                         0.7 ms

0:08  Summary:   1,204 verified   ·   3 unsupported   ·   0 rejected

0:09  "Nothing left your machine. This kernel has never seen Lean's source code."
```

That last line is true, it is the product, and it is the thing that makes a Mathlib maintainer stop scrolling.

### 6.1 Non-goals for the demo

No editor. No REPL. No tactics. No Lean *source* at all — it eats `.export` files and nothing else. **Every additional feature is another thing the demo has to be right about.** One thing, done completely.

---

## 7. Three buckets, always

Every verdict lands in exactly one of:

| Bucket | Meaning |
|---|---|
| **verified** | We checked it. It is a proof. |
| **unsupported** | We do not implement a feature it needs (§4). We say which one, by name. |
| **rejected** | We checked it and we say it is **not** a proof. |

Keep them separate everywhere — CLI, JSON report, demo, README, and any slide you ever make. This is not caution; it is **how every serious checker in this field reports**, and it is what makes the numbers usable by someone else.

The engineering reason is sharper than the social one: **`rejected` is an alarm.** If it is ever non-zero, either we have a kernel bug or we have found something in Lean, and we need to know *instantly* which conversation we are in. If `unsupported` and `rejected` are ever summed into one figure, that alarm is silently disabled. Wire them separately from day one, and make the CLI exit non-zero on any `rejected`.

**Publish the `unsupported` list, with reasons.** Every checker has one. `lean4lean` has one. Publishing it is what invites the community to close it *with* you — and in practice, that is how these things actually get finished.

---

## 8. Build hygiene

### 8.1 Fix the WASM export regression

`@cooljapan/oxilean@0.1.2` currently ships a `.wasm` whose export section contains one symbol (`memory`) and a `.d.ts` with no declarations. `0.1.1` exported 18 symbols. **A feature-flag or `#[wasm_bindgen]` gating change let dead-code elimination take the crate.**

Straightforward bug, quick fix. The important part is stopping it from recurring silently:

```bash
# web/scripts/export-gate.sh — CI, on every publish
n=$(wasm-objdump -x pkg/oxilean_verify_bg.wasm | grep -c '^ - func\[')
[ "$n" -ge 10 ] || { echo "wasm exports collapsed to $n — DCE ate the crate"; exit 1; }
```

An empty artifact is the one bug that produces no error anywhere until a user hits it. Gate it once, forget it forever.

### 8.2 Budgets

| | Budget |
|---|---|
| `oxilean-verify.wasm` | **≤ 400 kB gzip** — a zero-dep kernel should be small; if it isn't, something else linked in |
| Cold start (fetch + instantiate) | ≤ 300 ms |
| Throughput | within **5×** of `lean4lean` on the same corpus, same machine. Measure their baseline first; do not guess it. |

### 8.3 Gates

- **Zero-dependency invariant:** `cargo tree -p oxilean-kernel` shows **zero external crates.** This is the TCB and it is the product. Gate it in CI.
- **Dependency allow-list:** `oxilean-kernel` + `oxilean-export` + `wasm-bindgen` only. Any addition is a deliberate, reviewed edit to `verify/allowed-deps.txt`.
- **`#![forbid(unsafe_code)]`** in kernel and export reader.
- **Fuzzing:** the export reader parses untrusted input directly into the TCB. `cargo-fuzz`, in CI, non-negotiable.
- **Determinism:** identical verdicts and identical reports, native and WASM. Gate it.
- **Static server:** demo runs under `python3 -m http.server`.

---

## 9. Milestones

| # | Milestone | Done when |
|---|---|---|
| **M0** | **Foundations** *(≈1 week)* | WASM export gate. Dep allow-list gate. Zero-dep invariant gate. `0.1.3` published and working. Differential harness skeleton against `lean4lean`. |
| **M1** | **Kernel gap audit** *(≈2 weeks)* | §4 walked end to end. Quotients, structure eta, recursor re-derivation, positivity, level `isDefEq`: each **implemented**, or **explicitly out of scope in writing** with the reason. The bignum design chosen. This is the week that determines the whole schedule — do it properly. |
| **M2** | **`oxilean-export` + Lean core** *(≈2 weeks)* | Reader done, fuzzed, zero deps. **Verifies Lean 4 core's own export, end to end.** First real number: *"we verify X of Lean core's Y declarations; here are the Y−X and why."* Core is a fraction of Mathlib's size — this is the honest, achievable first win, and it is a genuine one. |
| **M3** | **Bignum + literal reduction** *(≈2 weeks)* | §4.3. Differential-tested, fuzzed, small enough to audit by eye. Unlocks the `decide`/`norm_num` corpus. |
| **M4** | **Mathlib** *(≈2–3 weeks)* | Run against real Mathlib exports. Publish three numbers and the full `unsupported` list. **Whatever the coverage is, publish it** — a partial checker with a stated boundary is a respected object in this field; there is no version of this where an honest number is a bad outcome. |
| **M5** | **Kernel in a Tab** *(≈1 week)* | §6. WASM. ≤ 400 kB. Ship. |
| **M6** | **Tell them** | §10. |

**≈10 weeks, one engineer.** Small, bounded, and every milestone produces something real.

---

## 10. Where this lands

The venue is the **Lean Zulip**, and the register is *"I built a thing, here is what it does, here is a question"* — because that is what actually gets read there, and because it happens to be true.

> *I've written an independent Lean 4 kernel in Rust — zero external dependencies, no `unsafe`, checks `lean4export` output. It currently verifies **N of M** declarations in Mathlib; the **M − N** are listed here with reasons, mostly [X]. It also runs in a browser: [link].*
>
> *There are **three** declarations where I disagree with `lean4lean`. I'd like to work out which of us is wrong.*

That last line is the whole post. It turns a room of reviewers into a room of collaborators, and it is exactly what a person who has just written a kernel actually wants to know.

**What success looks like:** not downloads. **A named person in that community, in public, treating our numbers as reliable.** That is a thing that cannot be bought, cannot be manufactured, and — see §11 — is worth more to this ecosystem than any other single output it could produce this year.

Hacker News, if at all, comes *after* that thread has gone well. Never before.

### 10.1 One communications note

The `99.7%` figure is a real result and a considerable one — 181,890 declarations is a serious corpus. It is correctly labelled in the README as **parse** compatibility, with the normalisation pipeline documented and Track 1 / Track 2 kept separate. That is right.

The only care needed is when the number **travels**: README → article → slide → teaser → someone else's tweet. Adjectives fall off in transit, and the one that must not is **"parse"**. Weld it on wherever the figure appears, especially in the places where you most want to compress. Precision is the currency in this room, and we are already paying in it — just make sure the change follows the number.

---

## 11. Why this project, and not another

Everything else in COOLJAPAN is measured by adoption. This one isn't. **This one is measured in credibility, and credibility is currently the ecosystem's scarcest resource.**

25M lines of Rust is an extraordinary artefact, and its central problem is that nobody outside has checked any of it. Benchmarks help. Users help more. But there is exactly one output that settles the question in a single sentence:

> **"Our kernel independently verified N of Mathlib's M declarations from Lean's own export. Here is the harness. Here are the ones we failed, and why. Run it yourself."**

A stranger can check that in ten minutes. It holds or it doesn't. **No other project in the portfolio can produce a sentence like that**, because no other project has a zero-dependency TCB pointed at a corpus the whole world already trusts.

That is why OxiLean is worth doing next, even though its audience is the smallest. **It isn't the loudest thing you own. It is the one that makes everything else you say believable.**

---

## 12. Open questions to answer early

Not risks — **things to find out in the first fortnight**, because each one changes the plan:

1. **Does the kernel already have quotients?** If not, how deep is the change? *(M1, day 1.)*
2. **Structure eta — present, absent, or partially present?** Partial is the dangerous one; it fails late and confusingly.
3. **How far does Lean core get without literal reduction?** If core verifies substantially without it, M2 lands early and M3 can be scheduled honestly.
4. **What does `lean4lean` actually cost, per declaration, on this hardware?** Get the real baseline before setting our own target. Do not guess it.
5. **Which recursor cases does the export actually exercise in practice?** Nested and mutual inductives are the expensive ones to support; find out how much of Mathlib genuinely needs them before building for the general case.

---

## Appendix — the project in five lines

1. Read `lean4export`. *(one week)*
2. Close the kernel gaps: quotients, structure eta, recursors, levels, bignum. *(the actual work — and the good part)*
3. Verify Lean core. Then Mathlib. Publish three numbers and the failure list.
4. Put it in a browser tab.
5. Post it on the Zulip with a question, not a claim.

**A second kernel that agrees. That's the whole thing.**

---

*Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.*
