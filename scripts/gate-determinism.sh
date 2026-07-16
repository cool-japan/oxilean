#!/usr/bin/env bash
# gate-determinism.sh — G7 (partial): the native half of the native-vs-wasm
# determinism check.
#
# The brief (§8.3) requires identical verdicts and identical reports from the
# native checker and the WASM checker on a fixture corpus. Both drive the exact
# same `oxilean_verify::verify_stream` engine over the exact same reader and
# kernel, so the per-declaration verdicts and the JSON report are deterministic
# by construction (only the timing fields are non-deterministic, and the report
# masks them to whole ms).
#
# This script implements the NATIVE half fully:
#   * runs the release `oxilean-verify` binary on the committed simple_add
#     fixture twice, with timing masked, and asserts the two JSON reports are
#     byte-identical, and
#   * asserts the three-bucket totals match the pinned expected values (the same
#     numbers the wasm engine produces, since it is the same engine).
#
# The WASM half (instantiate the .wasm under a JS runtime, run the same fixture,
# diff the report) is DEFERRED to Wave 4: this environment has no node / deno /
# headless browser, so the wasm report cannot be produced here. The wasm module
# is instead validated structurally (`wasm-tools validate`) and by kernel-symbol
# presence, and the engine path is proven identical natively here. See
# TODO_VERIFY.md G7.
#
# Usage:  bash scripts/gate-determinism.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="$ROOT/target/release/oxilean-verify"
FIXTURE="$ROOT/tests/fixtures/lean4export/simple_add.ndjson"

# Pinned expected three-bucket totals for simple_add (same as the CLI integration
# fixture table and oxilean-export's replay_smoke; verified through the real
# binary). If the engine changes coverage, update this AND the fixture table.
EXPECT_VERIFIED=23
EXPECT_UNSUPPORTED=0
EXPECT_REJECTED=0

if [ ! -x "$BIN" ]; then
    echo "--- Building release oxilean-verify binary ---"
    ( cd "$ROOT" && cargo build -p oxilean-verify --release )
fi
if [ ! -f "$FIXTURE" ]; then
    echo "FAIL: fixture missing: $FIXTURE" >&2
    exit 1
fi

# Mask the non-deterministic timing field (totals.wall_ms) so two runs compare
# byte-for-byte. Everything else in the report is deterministic.
mask_report() {
    "$BIN" --json - --quiet "$FIXTURE" 2>/dev/null \
        | sed 's/"wall_ms": [0-9]*/"wall_ms": 0/'
}

echo "--- Native determinism: two runs must be byte-identical (timing masked) ---"
R1="$(mask_report)"
R2="$(mask_report)"

if [ "$R1" != "$R2" ]; then
    echo "FAIL: two native runs produced different reports:" >&2
    diff <(printf '%s' "$R1") <(printf '%s' "$R2") >&2 || true
    exit 1
fi
echo "OK: two native runs are byte-identical."

echo "--- Native three-bucket totals match the pinned expectation ---"
get() { printf '%s' "$R1" | grep -oE "\"$1\": [0-9]+" | head -1 | grep -oE '[0-9]+'; }
V="$(get verified)"; U="$(get unsupported)"; REJ="$(get rejected)"

FAILED=0
[ "$V"   = "$EXPECT_VERIFIED" ]    || { echo "FAIL: verified=$V expected $EXPECT_VERIFIED"; FAILED=1; }
[ "$U"   = "$EXPECT_UNSUPPORTED" ] || { echo "FAIL: unsupported=$U expected $EXPECT_UNSUPPORTED"; FAILED=1; }
[ "$REJ" = "$EXPECT_REJECTED" ]    || { echo "FAIL: rejected=$REJ expected $EXPECT_REJECTED"; FAILED=1; }

if [ "$FAILED" -ne 0 ]; then
    exit 1
fi
echo "OK: verified=$V unsupported=$U rejected=$REJ (matches pinned expectation)."

echo ""
echo "Native determinism half PASSED."
echo "NOTE: the WASM half (run the same fixture through the .wasm and diff the"
echo "      report) is DEFERRED to Wave 4 — no node/deno/headless browser in"
echo "      this environment. The wasm module is validated structurally"
echo "      (wasm-tools validate) and drives the identical engine path."
exit 0
