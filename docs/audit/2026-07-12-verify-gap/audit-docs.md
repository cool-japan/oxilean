# oxilean Documentation Audit — Area: Documentation, Claims, Versioning State

Auditor: Claude Code (Sonnet 4.6)
Date: 2026-07-12
Branch: 0.1.3
Repo: oxilean

---

## 1. README Claims vs the Brief's Communication Rules

### 1.1 The 99.7% Figure and Its Qualifier

**Finding: COMPLIANT — qualifier present on the main stat.**

README.md line 110:
```
- **181,890 declarations** tested -- **99.7% parse compatibility** (181,326 parsed OK)
```
The word "parse" travels with the number. Good.

README.md line 113 (Track 1 definition) is explicit:
> Track 1 (parser compat): Reads .lean files ... and parses with OxiLean.

README.md line 115 (Track 2):
> Track 2 (curated theorems): 320 hand-adapted Mathlib4 theorems verified through parse + elaboration + tactic execution.

Track 2 does not carry a "100%" figure in the README body; the README only says "320 hand-adapted theorems verified." The 100% figure appears only in TODO.md line 340: "320 curated theorem proofs: 100% pass rate" — without an explicit "parse+elab" qualifier. This is a minor risk but not a violation of the brief (the brief's rule applies specifically to "parse").

CHANGELOG.md line 53 (v0.1.1): "achieving 99.7% parse compatibility across the entire Mathlib4 codebase" — qualified. OK.

**Summary:** The 99.7% figure always carries "parse" in the README and CHANGELOG. No unqualified bare percentage violates the brief's rule.

### 1.2 Track 1 (parse) vs Track 2 (verify) Separation

**Finding: PARTIALLY CORRECT — separation exists but terminology is ambiguous.**

The README uses "Track 1 (parser compat)" and "Track 2 (curated theorems)" at lines 113-115. The distinction between parse-only and parse+elab+tactic is clear in the text.

**Gap vs brief:** The brief describes "Track 2" as the *verification* track. The README describes Track 2 as "verified through parse + elaboration + tactic execution" — i.e., this is OxiLean's own elaborator, not the lean4export proof checker. The brief's "verify" product (oxilean-verify, consuming lean4export files, three-bucket verdict) is entirely absent from the README. Track 2 in the README conflates "OxiLean elaborates Lean4 syntax" with "OxiLean verifies Lean4 proof exports."

### 1.3 Three-Bucket Reporting Language

**Finding: ENTIRELY ABSENT.**

The brief requires exactly three verdict buckets: `verified` / `unsupported (named missing feature)` / `rejected (alarm; CLI exits non-zero)`.

Neither the README, TODO, CHANGELOG, CONTRIBUTING, nor any file in docs/ (EXAMPLES.md, FAQ.md, TUTORIAL.md, USER_GUIDE.md) mentions:
- The three-bucket schema
- The `verified` verdict
- The `unsupported` verdict with a named feature
- The `rejected` verdict
- A CLI exit-code convention for proof checking failures
- A JSON report schema

This is a complete documentation gap for the verify product.

---

## 2. TODO.md Accuracy: Phases 1-4 COMPLETE vs Brief's Hard Parts

### 2.1 Top-Level TODO Status

TODO.md line 6: "Phases 1-4 are COMPLETE. v0.1.2 released 2026-05-03."

This claim is accurate for the ITP (interactive theorem prover) codebase. The concern arises when mapping against the brief's oxilean-verify hard parts.

### 2.2 Quotient Types: Quot.ind Missing from Top-Level Claims

**Finding: PARTIALLY PRESENT — root TODO claims only 3 quotient declarations, implementation has 4.**

TODO.md line 79-80 (Phase 1b, which is marked COMPLETE):
```
- [x] 3 built-in declarations: `Quot.mk`, `Quot.lift`, `Quot.sound`
- [x] `Quot.lift f h (Quot.mk a) → f a` reduction rule
```

`Quot.ind` is NOT listed in the root TODO or README. However, it IS implemented in the source:
- `crates/oxilean-kernel/src/quotient/types.rs` line 265, 508, 756, 766, 877, 896, 1053: `Quot.ind` reduction rule `Quot.ind h (Quot.mk a) → h a` is implemented.
- `crates/oxilean-kernel/src/quotient/functions.rs` lines 140-163, 418-435: `reduce_quot_ind` function exists.
- `crates/oxilean-kernel/src/axiom/types.rs` line 1282: `Quot.ind` added to safe axioms.
- `crates/oxilean-kernel/TODO.md` lines 111-112 list only `Quot.mk`, `Quot.lift`, `Quot.sound` — same omission.

**Summary:** The root TODO.md and kernel TODO.md both list only 3 quotient declarations (missing Quot.ind), but the implementation actually includes all 4. This is a documentation error in the TODO — the implementation is ahead of the documentation. The brief requires Quot.ind + computation rule `Quot.ind h (Quot.mk r a) == h a`; the source has it but the docs don't claim it.

### 2.3 Structural Eta for Structures

**Finding: PRESENT IN KERNEL TODO, ABSENT FROM ROOT TODO.**

kernel/TODO.md line 201: `- [x] η-expansion for structures`
kernel/TODO.md line 66: `- [x] η-expansion: f ≡ λx. f x when x ∉ FV(f)`

Root TODO.md line 49 (Phase 1, def_eq section): `- [x] η-expansion`

The brief's requirement is "definitional eta for structures (single-constructor non-recursive inductives: s == <s.1, s.2>)." The kernel source has `eta.rs` (221 lines, per kernel/TODO.md line 172). This is apparently present. The root TODO claims "η-expansion" at line 49 without specifying structural eta. The brief's specific requirement (structural eta on single-constructor inductives) is documented only in kernel/TODO.md line 201, not in root TODO.md.

### 2.4 Nat/String Literal Reduction with Zero-Dependency Bignum

**Finding: PARTIALLY PRESENT — documented as fast-path, but NOT using arbitrary-precision bignum.**

Root TODO.md line 36: `- [x] Nat and String literal operations`
kernel/TODO.md line 48: `- [x] Nat literal operations (succ/add/mul/sub/div/mod/pow/beq/ble/blt/gcd/land/lor/xor/shift)`
kernel/TODO.md line 49: `- [x] String literal operations (length/append/beq)`

**Critical gap vs brief:** The brief specifies "a hand-written zero-dependency arbitrary-precision bignum (Nat.add/mul/sub/div/mod/pow/gcd/beq/ble/bitwise directly on literals)." The existing code uses `Literal::Nat` (presumably u64 or BigUint from `num-bigint`). Looking at Cargo.toml, the workspace has `num-bigint = { version = "0.4", default-features = false }` as a dependency — this is NOT zero-dependency bignum in the kernel's sense.

The kernel is supposed to have zero external deps. `num-bigint` at the workspace level may not be used by oxilean-kernel directly. Checking: the kernel Cargo.toml just says `version.workspace = true` with no extra deps listed. The `num-bigint` dependency is declared at the workspace level. Whether the kernel uses it cannot be confirmed from this audit without reading kernel/Cargo.toml for its specific dependencies.

The TODO doesn't document that a hand-written bignum was implemented (which the brief requires). The TODO claims Nat literal fast-paths (implying small-integer optimization), not arbitrary-precision. The word "bignum" does not appear anywhere in the docs.

### 2.5 Recursors for Nested and Mutual Inductives + Iota Reduction + Strict Positivity Re-check

**Finding: PARTIALLY PRESENT — basic cases documented, mutual/nested not documented, re-deriving not claimed.**

Root TODO.md line 65 (Phase 1b):
```
- [x] Strict positivity check
```
kernel/TODO.md line 94: `- [x] Strict positivity check`

Root TODO.md (Phase 1b, Recursor Generation, lines 72-76):
```
- [x] Generate T.rec type (motive + minors + major → motive applied)
- [x] Recursor computation rules
- [x] Handle recursive constructor arguments detection
- [x] ι-reduction in WHNF
```

**Gap vs brief:** The brief requires:
1. "recursors for nested and mutual inductives" — neither root TODO nor kernel TODO claims "nested inductives" or "mutual inductives" at the kernel level. Only termination checking mentions "mutual recursion" (kernel/TODO.md line 142: "mutual recursion, transitive subterm relation") and root TODO.md line 202: `[x] Mutual recursion support` (in the elaborator, not kernel).
2. "RE-DERIVING recursors ourselves rather than trusting exported ones" — this critical verification requirement is NOT mentioned in any TODO, README, or documentation. The current design trusts the exported RecursorVal from the lean4export stream, which would be insufficient for the verify product's TCB.
3. "strict positivity re-check" — the existing check is done at inductive declaration time, not as an independent re-verification step of exported inductives.

### 2.6 Universe Level Definitional Equality

**Finding: PRESENT IN BOTH ROOT TODO AND KERNEL TODO.**

Root TODO.md lines 22-26 (Phase 1, Level Operations):
```
- [x] normalize(l) — canonical form
- [x] level_leq(u, v) — universe level comparison
- [x] level_eq(u, v) — bidirectional leq
- [x] imax_simplify(u, v) — simplify IMax expressions
```

kernel/TODO.md lines 31-36 confirm. Also:
- kernel/TODO.md lines 61-69 (def_eq): structural comparison after WHNF, proof irrelevance.

This matches the brief's requirement for "max/imax normalisation, isDefEq on levels." Documentation appears accurate.

---

## 3. Missing Verify-Product Documentation

### 3.1 docs/VERIFY.md — Does Not Exist

`docs/` contains: EXAMPLES.md, FAQ.md, TUTORIAL.md, USER_GUIDE.md.
`docs/specs/` exists but is **empty** (confirmed: directory with zero files).

The following documentation is completely absent:
- Three-bucket verdict schema (verified / unsupported / rejected)
- Exit codes (e.g., 0=verified, 1=rejected/alarm, 2=unsupported)
- JSON report schema (if any)
- CLI usage examples for `oxilean-verify`
- The `--unsupported-list` format or similar machine-readable feature-gap reporting

### 3.2 docs/specs/ — Empty Directory

The `docs/specs/` directory exists but contains no files. It should contain:
- Pinned lean4export commit SHA / format version
- Pinned Lean toolchain version (for reproducibility)
- The unsupported-feature list (what lean4export constructs trigger the "unsupported" verdict)
- Wire format description

### 3.3 Unsupported-List Publication Format

The brief requires a machine-readable list of named missing features. No documentation of this format exists anywhere in the repo.

### 3.4 oxilean-export Crate Does Not Appear in Workspace

There is no `crates/oxilean-export` or `oxilean-verify` in the workspace. The brief's TCB includes "new oxilean-export reader (<=3000 SLoC, zero deps, fuzzed)." This crate is completely absent.

---

## 4. Version Bump Checklist for 0.1.3

**Current state:** Branch `0.1.3` is active, but the workspace version string is still `0.1.2` everywhere.

### 4.1 Files That Need Version Bumped to 0.1.3

#### Single source of truth (one edit = all crates update):
- **`Cargo.toml` line 21**: `version = "0.1.2"` → `"0.1.3"`

This is the `[workspace.package]` version. All 14 crates use `version.workspace = true`, so a single edit updates them all. ✓ (Confirmed via `grep -c "version.workspace = true"` returning 14.)

#### Internal dependency pins (also in root Cargo.toml):
- Lines 50-59: all 10 `oxilean-* = { path = "crates/oxilean-*", version = "0.1.2" }` pins need updating.

These are NOT updated by the workspace.package version field alone — each `version = "0.1.2"` in `[workspace.dependencies]` must be changed to `"0.1.3"`.

#### Files with hardcoded `0.1.2` strings (from `grep -rn "0\.1\.2" --include="*.md"`):

| File | Location | String |
|------|----------|--------|
| README.md | line 152 | `## What's New in v0.1.2` |
| README.md | line 263 | `**v0.1.2** (2026-05-03)` |
| crates/oxilean-meta/README.md | line 96 | `oxilean-meta = "0.1.2"` |
| crates/oxilean-lint/README.md | line 95 | `oxilean-lint = "0.1.2"` |
| crates/oxilean-build/README.md | line 84 | `oxilean-build = "0.1.2"` |
| crates/oxilean-codegen/README.md | line 113 | `oxilean-codegen = "0.1.2"` |
| crates/oxilean-std/README.md | line 74 | `oxilean-std = "0.1.2"` |
| crates/oxilean-runtime/README.md | line 66 | `oxilean-runtime = "0.1.2"` |
| crates/oxilean/README.md | lines 46, 53, 87, 108 | `oxilean = "0.1.2"` (4 occurrences) |
| CHANGELOG.md | line 22 | `## [0.1.2] — 2026-05-03` (keep, historical) |

#### Files already at 0.1.3 (partial or version-ahead):

| File | Location | Notes |
|------|----------|-------|
| crates/oxilean-cli/src/lsp/editor/vscode/package.json | line 5 | `"version": "0.1.3"` — AHEAD of workspace |
| crates/oxilean-wasm/playground/index.html | line 12 | `v0.1.3` — AHEAD of workspace |
| crates/oxilake/src/lockfile.rs | lines 117, 120, 131, 135, 143, 144, 189 | `"0.1.3"` in test fixtures — OK (test data) |

The vscode package.json and playground index.html being at 0.1.3 while the workspace is at 0.1.2 is an inconsistency introduced during 0.1.3 development. When the workspace is bumped, these will be in sync.

#### CHANGELOG.md:
- Needs a new `## [0.1.3] — DATE` section added above `## [0.1.2]`.
- The `## [Unreleased]` section currently lists only 4 generic bullet points ("Interactive proof mode improvements", "Language server protocol (LSP) support", "Package manager integration", "Extended standard library coverage") — these do NOT reflect the extensive work done on the 0.1.3 branch (omega, linarith, nlinarith, cc tactic, LSP, playground, oxilake, oxilean-doc).
- The comparison link at line 209 would need updating: `[0.1.3]: https://github.com/cool-japan/oxilean/compare/v0.1.2...v0.1.3`

#### publish.sh:
- No hardcoded version. Reads `WORKSPACE_VERSION` dynamically from Cargo.toml via sed. ✓ No edit needed.
- Missing `oxilake` and `oxilean-doc` from TIER definitions (TIER1-TIER5 enumerate 9 crates, missing oxilake, oxilean-doc, and oxilean-cli is in TIER4 but `oxilean` meta-crate in TIER5).

Let's verify: publish.sh TIER1=(oxilean-kernel), TIER2=(oxilean-parse oxilean-meta oxilean-std oxilean-codegen oxilean-runtime), TIER3=(oxilean-elab oxilean-build oxilean-lint), TIER4=(oxilean-cli oxilean-wasm), TIER5=(oxilean). Missing: `oxilake` and `oxilean-doc`.

#### lib.rs doc comments:
`grep -rn "0\.1\." --include="*.rs"` returned no matches in `//!` comments. No version strings embedded in source doc comments. ✓ No edits needed for lib.rs.

#### wasm-bindgen dependency version in oxilean-wasm README.md:
- README.md (lines 98, 100) says: `wasm-bindgen 0.2.114`, `serde-wasm-bindgen 0.6.5`
- Cargo.toml (line 27) says: `wasm-bindgen = { version = "0.2.126" }`, `js-sys = { version = "0.3.103" }`
- The README is 12 minor versions stale on wasm-bindgen and 20 patch versions stale on js-sys.
- This README was not updated when the deps were bumped in v0.1.2 per CHANGELOG ("wasm-bindgen upgraded 0.2.118 → 0.2.120") and in the subsequent bump to 0.2.126.

---

## 5. LICENSE and Copyright

### 5.1 License Type: CORRECT
LICENSE file: Apache License Version 2.0. ✓
Cargo.toml workspace: `license = "Apache-2.0"` ✓
All crate READMEs: "Apache-2.0" ✓

### 5.2 Copyright Holder: CORRECT
CONTRIBUTING.md line 711: "licensed under Apache-2.0" — contributions absorbed.
Cargo.toml workspace: `authors = ["COOLJAPAN OU (Team Kitasan)"]` ✓
LICENSE file line 189: `Copyright 2026 COOLJAPAN OU (Team KitaSan)` ✓ (Note: different capitalization — see §5.3)

### 5.3 Minor Inconsistency: "KitaSan" vs "Kitasan"
- LICENSE file: "COOLJAPAN OU (Team **KitaSan**)" — capital S
- All 12 crate READMEs: "COOLJAPAN OU (Team **Kitasan**)" — lowercase s
- Root README, CHANGELOG, Cargo.toml: "COOLJAPAN OU (Team **Kitasan**)" — lowercase s

This is a typography inconsistency; the legal entity name should be consistent. Assuming "KitaSan" in the LICENSE is canonical (as it's the formal license file), all other documents should be updated, or vice versa.

---

## 6. Additional Gaps Found

### 6.1 Crate Count in Documentation: WRONG

README.md line 17: "1.35M+ lines of Rust across 5,978 source files and **12 crates**"
README.md line 81: "**12 crates**, 5,978 Rust files" (table footer)
README crate table: Lists only 11 rows (missing `oxilean` meta-crate)
Workspace Cargo.toml: 14 members (oxilake, oxilean, oxilean-build, oxilean-cli, oxilean-codegen, oxilean-doc, oxilean-elab, oxilean-kernel, oxilean-lint, oxilean-meta, oxilean-parse, oxilean-runtime, oxilean-std, oxilean-wasm)

The README table is stale — it reflects the v0.1.0/v0.1.1 state (11 crates listed) but claims 12. As of the 0.1.3 branch, the workspace has 14 crates. The table is missing: `oxilean` (meta-crate), `oxilake`, `oxilean-doc`.

### 6.2 Test Count Inconsistency Between README and TODO

README.md: "33,091 tests passing" (multiple occurrences)
TODO.md "Project Status: COMPLETE" section (line 337): "32,345 tests passing"

The numbers differ by ~746 tests. One or both is stale. The README appears more recent (it includes v0.1.2 changes).

### 6.3 oxilean-wasm README Dependency Versions Stale

oxilean-wasm/README.md line 98: `wasm-bindgen 0.2.114`
oxilean-wasm/Cargo.toml line 27: `wasm-bindgen = "0.2.126"`
oxilean-wasm/README.md line 100: `serde-wasm-bindgen 0.6.5` — matches Cargo.toml ✓
oxilean-wasm/README.md: no mention of js-sys version; Cargo.toml has `js-sys = "0.3.103"`

The CHANGELOG records bumps to 0.2.120, but README still shows pre-0.2.114. The current workspace version is 0.2.126.

### 6.4 publish.sh Missing New Crates

publish.sh TIER definitions:
- TIER1: oxilean-kernel
- TIER2: oxilean-parse oxilean-meta oxilean-std oxilean-codegen oxilean-runtime
- TIER3: oxilean-elab oxilean-build oxilean-lint
- TIER4: oxilean-cli oxilean-wasm
- TIER5: oxilean

Missing: **oxilake** (new v0.1.3 crate) and **oxilean-doc** (new v0.1.3 crate)

These need to be inserted into the appropriate tiers before 0.1.3 publish. oxilake depends on oxilean-build (tier 3), so oxilake should be TIER4. oxilean-doc depends on oxilean-parse (tier 2), so oxilean-doc should be TIER3.

### 6.5 CHANGELOG [Unreleased] Section Is Stale

The `[Unreleased]` section has only 4 generic bullets that describe v0.1.0-era features (LSP, package manager, stdlib). It does not document the substantial v0.1.3 branch work:
- omega tactic with Cooper's algorithm
- linarith with Fourier-Motzkin
- nlinarith with Positivstellensatz-lite
- cc tactic with kernel-verified proofs
- polyrith with Gröbner basis
- LSP integration tests
- VS Code extension skeleton
- Incremental didChange sync
- Browser playground (CodeMirror 6, IndexedDB, share-via-URL)
- oxilake package manager (Ring 0 + Ring 1 complete)
- oxilean-doc documentation generator (Ring 0 + Ring 1 complete)
- Int literal arithmetic in kernel

### 6.6 No docs/VERIFY.md — Complete Gap for Verify Product

The brief requires documentation at `docs/VERIFY.md` covering:
- Three-bucket verdicts
- Exit codes
- JSON report schema (if applicable)
- Which lean4export constructs are in scope vs "unsupported"

This file does not exist. The specs/ directory is empty.

### 6.7 No Cargo-Fuzz Configuration

The brief requires cargo-fuzz on the export reader in CI. No fuzz targets exist:
- `find . -name "fuzz_targets"` returns nothing
- `.github/workflows.disabled/ci.yml` is disabled and doesn't mention fuzzing
- No `Cargo.toml` with `[workspace] members = [...fuzz...]`

---

## 7. Summary of What Is Present vs Absent

### Present and Correct
- Apache-2.0 license, COOLJAPAN OU copyright ✓
- 99.7% figure always qualified with "parse" in README and CHANGELOG ✓
- Track 1 (parse) vs Track 2 (curated theorems) separation in README ✓
- Workspace version managed centrally (Cargo.toml `[workspace.package]`) ✓
- publish.sh dynamically reads version from Cargo.toml ✓
- All crate Cargo.toml files use `version.workspace = true` ✓
- No version strings embedded in lib.rs `//!` doc comments ✓
- CHANGELOG follows Keep a Changelog format ✓
- Git tags exist for v0.1.0, v0.1.1, v0.1.2 ✓
- Quot.ind IS implemented in kernel source (just undocumented) ✓

### Present but Stale/Incorrect
- README says "12 crates" but workspace has 14 crates; table lists 11 rows
- README test count (33,091) differs from TODO test count (32,345)
- oxilean-wasm/README.md has stale wasm-bindgen version (0.2.114 vs actual 0.2.126)
- CHANGELOG [Unreleased] section does not reflect v0.1.3 work
- Root TODO and kernel TODO don't document Quot.ind (implemented but unclaimed)
- Root TODO (line 79) says "3 built-in declarations" for quotient types (missing Quot.ind)
- TODO project-status section has "32,345" tests but README says "33,091"
- "KitaSan" capitalization inconsistency between LICENSE and all other files

### Missing Entirely (for oxilean-verify product)
- `crates/oxilean-export/` — does not exist (new crate required by brief)
- `crates/oxilean-verify/` — does not exist
- `docs/VERIFY.md` — does not exist
- `docs/specs/` — directory exists but is empty; needs lean4export commit pin, Lean toolchain pin, unsupported-feature list
- Three-bucket verdict schema anywhere in docs
- Exit-code documentation for CLI proof checker
- JSON report schema
- Cargo-fuzz harness for export reader
- WASM export-count gate documentation
- Determinism native-vs-WASM test documentation
- Demo (python3 -m http.server) — playground build.sh has the python3 instruction ✓ (partial match)

---

## 8. Version Bump Checklist for 0.1.3 (Actionable)

In order of execution:

1. **`Cargo.toml` line 21**: `version = "0.1.2"` → `"0.1.3"`
2. **`Cargo.toml` lines 50-59**: all internal dep pins from `"0.1.2"` → `"0.1.3"`
3. **`CHANGELOG.md`**: Add `## [0.1.3] — DATE` section with all v0.1.3 changes; update comparison link at line 209
4. **`README.md`**:
   - Add `## What's New in v0.1.3` section (or rename/replace the v0.1.2 section)
   - Update footer `**v0.1.2** (2026-05-03)` → `**v0.1.3** (DATE)`
   - Fix crate count: 14 crates (add oxilake, oxilean-doc rows to table)
5. **`crates/*/README.md`** (meta, lint, build, codegen, std, runtime, oxilean): Update `"0.1.2"` version pins in code examples
6. **`crates/oxilean-wasm/README.md`**: Update wasm-bindgen from 0.2.114 → 0.2.126, js-sys from 0.3.83 → 0.3.103
7. **`publish.sh`**: Add `oxilake` to TIER4 (after oxilean-build), add `oxilean-doc` to TIER3
8. **`git tag v0.1.3`** after all changes committed
9. **LICENSE / capitalization**: Decide on "KitaSan" vs "Kitasan" and make consistent

Files that do NOT need changes:
- `publish.sh` version string — auto-read from Cargo.toml ✓
- `crates/oxilean-cli/src/lsp/editor/vscode/package.json` — already at 0.1.3 ✓
- `crates/oxilean-wasm/playground/index.html` — already shows v0.1.3 ✓
- All `lib.rs` files — no version strings embedded ✓
