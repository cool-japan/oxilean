# Audit Notes: lean4export Format Specification Fetch

**Date:** 2026-07-12  
**Auditor task:** Fetch and consolidate authoritative lean4export format spec  
**Output file:** `docs/specs/lean4export-format.md`  
**Scratchpad copy:** `<scratchpad>/lean4export-format.md`

---

## Sources Fetched

### 1. lean4export repo (primary authority)

| File | URL | HTTP Status | Confidence |
|---|---|---|---|
| `format_ndjson.md` | `https://raw.githubusercontent.com/leanprover/lean4export/master/format_ndjson.md` | 200 OK | HIGH — full content retrieved |
| `Export.lean` | `https://raw.githubusercontent.com/leanprover/lean4export/master/Export.lean` | 200 OK | HIGH — full source retrieved |
| `Export/Parse.lean` | `https://raw.githubusercontent.com/leanprover/lean4export/master/Export/Parse.lean` | 200 OK | HIGH — full source retrieved |
| `lean-toolchain` | `https://raw.githubusercontent.com/leanprover/lean4export/master/lean-toolchain` | 200 OK | HIGH |
| `lakefile.toml` | `https://raw.githubusercontent.com/leanprover/lean4export/master/lakefile.toml` | 200 OK | HIGH |
| GitHub API (latest commit) | `https://api.github.com/repos/leanprover/lean4export/commits?per_page=1` | 200 OK | HIGH |

**lean-toolchain content:** `leanprover/lean4:v4.32.0-rc1`  
**Latest commit:** `3de59f10bc4b4a0f2de698597aeb1246caa0df0a`  
  Message: "chore: bump toolchain to v4.32.0-rc1 (#39)"  
  Date: 2026-06-17T19:15:39Z

### 2. lean4lean repo (consumer-side reference)

| File | URL | HTTP Status | Confidence |
|---|---|---|---|
| `README.md` | `https://raw.githubusercontent.com/digama0/lean4lean/master/README.md` | 200 OK | HIGH — full content retrieved |
| GitHub API (latest commit) | `https://api.github.com/repos/digama0/lean4lean/commits?per_page=1` | 200 OK | HIGH |

**lean4lean latest commit:** `8865b155abbf68d3a827fb3568bf6839780163c2`  
  Message: "feat: dynamic fuel config"  
  Date: 2026-07-06T19:50:20Z

**Note on lean4lean format relevance:** lean4lean does NOT consume the lean4export NDJSON format.
It reads `.olean` binary files directly. Its relevance to oxilean-verify is as a reference
implementation for the type-checking algorithms (quotients, inductives, etc.), not for the
export format itself.

### 3. trepplein (not fetched)

trepplein consumes the OLD text-line lean4export format, not the current NDJSON format.
Since the current lean4export produces NDJSON (v3.1.0), trepplein format notes are not
directly relevant to the oxilean-export reader implementation. No fetch was performed.

---

## What Was Fetched vs. What Was Reconstructed

### Directly fetched (HIGH confidence):
- `format_ndjson.md`: the complete format spec document — all JSON schemas for all 17
  line types are exact as documented.
- `Export.lean`: the complete serializer source — provides ground truth for exact field
  names, ordering, and edge cases (nondep normalization, quotient ordering, literal deps).
- `Export/Parse.lean`: the complete reference parser — provides ground truth for how each
  line is parsed and how the index tables are initialized and queried.
- `lean-toolchain`: confirms `leanprover/lean4:v4.32.0-rc1`.
- Latest commit hash via GitHub API.

### Reconstructed from knowledge (FIXME-VERIFY items):
- **Version history:** Whether lean4export ever had a v1/v2 text-line format is NOT
  confirmed from fetched sources. The repo only documents format version 3.1.0.
  The old text-line format was used by earlier tools (lean-checker, trepplein).
  The spec file marks this section FIXME-VERIFY.
- **Trepplein old format grammar:** The `NS`, `NI`, `US`, etc. mnemonic codes for the
  old text format were reconstructed from knowledge, NOT fetched. Mark as FIXME-VERIFY
  if the old text format support is required.

---

## Key Findings for oxilean-verify Implementation

### Format is NDJSON (not text-line)
The current lean4export format (v3.1.0) is NDJSON. Every line is a JSON object.
This is a significant departure from the older text-line format used by trepplein/lean-checker.
The oxilean-export reader must be a JSON parser, not a space-delimited tokenizer.

### Index table initialization
- `nameMap[0] = Name.anonymous` (never emitted, pre-seeded)
- `levelMap[0] = Level.zero` (never emitted, pre-seeded)
- `exprMap` starts empty (first expr gets index 0 when emitted)

### Critical implementation detail: natVal as string
Natural number literals are emitted as decimal strings (`"natVal": "12345..."`), not
JSON numbers. This is required because Lean Nat values can be arbitrarily large and
would overflow JSON integers. The oxilean-export reader MUST use arbitrary-precision
arithmetic (brief section 4(3)).

### letE nondep normalization
When `--export-mdata` is NOT used (the default), the serializer normalizes `nondep`
to `false` in all `letE` expressions. This prevents spurious structural inequality.
The reader should NOT use `nondep` as a semantic field for type-checking.

### Quotient ordering guarantee
The exporter always emits all four quotient declarations in order: `Quot`, `Quot.mk`,
`Quot.lift`, `Quot.ind`. It also ensures `Eq` is exported first. The reader can rely
on this ordering.

### Inductive: exported recursors are informational
The spec explicitly states that exported recursors are redundant — a full checker
re-derives them. oxilean-verify must re-derive recursors from the `InductiveVal`
data (which includes `numParams`, `numIndices`, `ctors`, etc.) rather than trusting
the exported `RecursorVal.rules`.

### Field ordering in JSON objects
JSON objects are unordered. The reference parser normalizes key order before matching
but this is an implementation convenience — the spec does not guarantee field order.
The oxilean-export reader must use key-based lookup, not positional parsing.

---

## Confidence Summary

| Claim | Confidence | Source |
|---|---|---|
| Format version 3.1.0 is NDJSON | HIGH | format_ndjson.md (fetched) |
| All 17 line type schemas | HIGH | format_ndjson.md + Export.lean (fetched) |
| nameMap[0]=anonymous, levelMap[0]=zero | HIGH | Export/Parse.lean (fetched) |
| Toolchain: leanprover/lean4:v4.32.0-rc1 | HIGH | lean-toolchain (fetched) |
| Commit hash: 3de59f10... | HIGH | GitHub API (fetched) |
| natVal is decimal string | HIGH | Export.lean + format_ndjson.md (fetched) |
| nondep normalized to false | HIGH | Export.lean comment (fetched) |
| letE nondep is optimization hint only | HIGH | Export.lean comment (fetched) |
| Quot emits all 4 in order | HIGH | Export.lean (fetched) |
| Recursors are informational/redundant | HIGH | format_ndjson.md note (fetched) |
| Old text-line format grammar (NS/NI/etc) | MEDIUM | From knowledge, not fetched |
| Version history (v1/v2 text format) | LOW | FIXME-VERIFY, not confirmed |
| trepplein format compatibility | LOW | FIXME-VERIFY, not fetched |

---

## oxilean repo state (relevant to spec)

The existing oxilean repo has a `crates/oxilean-kernel/src/export/` directory but
it contains auto-generated stubs (auto-generated module structure comment, trait files,
etc.) — NOT an implementation of the lean4export NDJSON reader. A new `oxilean-export`
crate needs to be created per the brief.

Existing `crates/oxilean-parse/` parses OxiLean surface syntax (not lean4export format).
