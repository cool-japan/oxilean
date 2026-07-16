#!/usr/bin/env bash
# Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
#
# test_harness.sh — self-contained smoke test for the differential harness.
#
# This proves the join logic end-to-end WITHOUT needing lean4lean or a Lean
# toolchain: it feeds two hand-written normalized verdict files (a "mock
# adapter") through diff_verdicts.py and asserts the agree/disagree/skew
# classification is exactly right — including that a verified-vs-rejected
# contradiction is surfaced as a DISAGREE finding and trips exit 1.
#
# It also (when the release binary exists) runs run_oxilean.sh over a committed
# fixture and joins oxilean-against-oxilean, which must produce zero
# disagreements by construction.
#
# Usage:  bash verify/differential/test_harness.sh
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
DIFF="$HERE/diff_verdicts.py"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

fail() { echo "FAIL: $*" >&2; exit 1; }

echo "== differential harness smoke test =="

# --- 1. mock adapter: hand-written verdicts exercising every bucket ----------
cat > "$TMP/mock_a.jsonl" <<'JSONL'
{"name": "Both.verified",   "verdict": "verified",    "detail": null}
{"name": "Both.rejected",   "verdict": "rejected",    "detail": "type mismatch"}
{"name": "Contradiction",   "verdict": "verified",    "detail": null}
{"name": "A.unsupported",   "verdict": "unsupported", "detail": "Nat literal reduction"}
{"name": "Only.A",          "verdict": "verified",    "detail": null}
JSONL

cat > "$TMP/mock_b.jsonl" <<'JSONL'
{"name": "Both.verified",   "verdict": "verified", "detail": null}
{"name": "Both.rejected",   "verdict": "rejected", "detail": "kernel says no"}
{"name": "Contradiction",   "verdict": "rejected", "detail": "kernel says no"}
{"name": "A.unsupported",   "verdict": "verified", "detail": null}
{"name": "Only.B",          "verdict": "verified", "detail": null}
JSONL

set +e
OUT="$(python3 "$DIFF" --a-name oxilean --a "$TMP/mock_a.jsonl" \
                       --b-name lean4lean --b "$TMP/mock_b.jsonl" \
                       --out-md "$TMP/mock.md" --fail-on-disagree)"
rc=$?
set -e

echo "$OUT"

# Expect: 2 AGREE (Both.verified, Both.rejected), 1 DISAGREE (Contradiction),
# 2 ONLY-ONE-SIDE (Only.A, Only.B), 1 UNSUPPORTED-SKIPPED (A.unsupported).
echo "$OUT" | grep -q "AGREE (both verified or both rejected) : 2" \
    || fail "expected 2 AGREE"
echo "$OUT" | grep -q "DISAGREE (verified vs rejected)        : 1" \
    || fail "expected 1 DISAGREE"
echo "$OUT" | grep -q "ONLY-ONE-SIDE (version/toolchain skew) : 2" \
    || fail "expected 2 ONLY-ONE-SIDE"
echo "$OUT" | grep -q "UNSUPPORTED-SKIPPED (one side declines): 1" \
    || fail "expected 1 UNSUPPORTED-SKIPPED"
echo "$OUT" | grep -q "Contradiction: oxilean=verified lean4lean=rejected" \
    || fail "disagreement not surfaced with both verdicts"
[[ "$rc" -eq 1 ]] || fail "--fail-on-disagree must exit 1 when a disagreement exists (got $rc)"
grep -q "Contradiction" "$TMP/mock.md" || fail "markdown report missing the disagreement"
echo "OK: mock adapter -> every bucket classified, disagreement is a FINDING (exit 1)."

# --- 2. no-disagreement path must exit 0 -------------------------------------
set +e
python3 "$DIFF" --a-name oxilean --a "$TMP/mock_a.jsonl" \
                --b-name other --b "$TMP/mock_a.jsonl" \
                --fail-on-disagree >/dev/null
rc=$?
set -e
[[ "$rc" -eq 0 ]] || fail "identical inputs must exit 0 (got $rc)"
echo "OK: identical verdict sets agree everywhere (exit 0)."

# --- 3. oxilean-vs-oxilean on a real fixture (if the binary is built) --------
BIN="${OXILEAN_VERIFY:-$ROOT/target/release/oxilean-verify}"
FIXTURE="$ROOT/tests/fixtures/lean4export/simple_add.ndjson"
if [[ -x "$BIN" && -f "$FIXTURE" ]]; then
    OXILEAN_VERIFY="$BIN" bash "$HERE/run_oxilean.sh" "$FIXTURE" "$TMP/ox.jsonl" >/dev/null 2>&1
    set +e
    python3 "$DIFF" --a-name run1 --a "$TMP/ox.jsonl" \
                    --b-name run2 --b "$TMP/ox.jsonl" \
                    --fail-on-disagree >/dev/null
    rc=$?
    set -e
    [[ "$rc" -eq 0 ]] || fail "oxilean-vs-oxilean on simple_add must agree (got $rc)"
    echo "OK: run_oxilean.sh over simple_add joins against itself with zero disagreements."
else
    echo "SKIP: release binary or fixture missing; ran mock-adapter tests only."
fi

echo "== all differential harness smoke tests passed =="
