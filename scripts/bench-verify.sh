#!/usr/bin/env bash
# Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
#
# bench-verify.sh — measure oxilean-verify throughput (declarations/second) on
#   (a) the six committed fixtures, and
#   (b) a deterministic 2,000-declaration slice of Lean core's Init.
#
# This is the throughput leg of V6/V7 (engineering brief §8.2). The brief's
# budget is "within 5x of the lean4lean baseline"; this script produces OUR
# number. The lean4lean baseline is measured separately by the differential
# harness (see verify/differential/). No wrong verdict is ever accepted for
# speed: this only times the real, unmodified checker.
#
# Method: 3 timed runs per input, report the MEDIAN wall time and the resulting
# decls/sec (verified+unsupported+rejected over wall). Timing uses the
# nanosecond monotonic clock via python3 (stdlib). Machine specs are printed.
#
# Usage:
#   scripts/bench-verify.sh [--runs N] [--slice PATH] [--md OUT.md]
#
# Environment:
#   OXILEAN_VERIFY   path to the binary (default: target/release/oxilean-verify)

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${OXILEAN_VERIFY:-$ROOT/target/release/oxilean-verify}"
FIXTURE_DIR="$ROOT/tests/fixtures/lean4export"
SLICE="$ROOT/verify/differential/slices/init-2000.ndjson"
RUNS=3
MD_OUT=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --runs)  RUNS="$2"; shift 2 ;;
        --slice) SLICE="$2"; shift 2 ;;
        --md)    MD_OUT="$2"; shift 2 ;;
        *) echo "unknown argument: $1" >&2; exit 2 ;;
    esac
done

if [[ ! -x "$BIN" ]]; then
    echo "building release oxilean-verify ..." >&2
    ( cd "$ROOT" && cargo build --release -p oxilean-verify )
fi

# --- machine specs -----------------------------------------------------------
CPU="$(grep -m1 'model name' /proc/cpuinfo 2>/dev/null | cut -d: -f2 | sed 's/^ //' || echo unknown)"
CORES="$(nproc 2>/dev/null || echo '?')"
MEM_KB="$(grep -m1 MemTotal /proc/meminfo 2>/dev/null | awk '{print $2}' || echo 0)"
MEM_GB="$(python3 -c "print(f'{${MEM_KB:-0}/1024/1024:.1f}')")"
OS="$(uname -sr)"
RUSTC="$(rustc --version 2>/dev/null || echo unknown)"

# Bench one input: run the binary $RUNS times, collect (decls, wall_ns) per run,
# print a result line, and append a markdown table row to $MD_OUT if set.
# Uses python3 for both the timing loop and the median so there is no floating
# point in the shell.
bench_one() {
    local label="$1" path="$2"
    if [[ ! -f "$path" ]]; then
        echo "  SKIP $label (missing: $path)" >&2
        return
    fi
    OXILEAN_VERIFY="$BIN" RUNS="$RUNS" LABEL="$label" MD_OUT="$MD_OUT" \
        python3 - "$path" <<'PY'
import json, os, subprocess, sys, time

path = sys.argv[1]
binp = os.environ["OXILEAN_VERIFY"]
runs = int(os.environ["RUNS"])
label = os.environ["LABEL"]
md_out = os.environ.get("MD_OUT", "")

walls = []
decls = None
for _ in range(runs):
    t0 = time.monotonic_ns()
    proc = subprocess.run(
        [binp, "--quiet", "--no-color", "--limits", "corpus", "--json", "-", path],
        capture_output=True, text=True,
    )
    t1 = time.monotonic_ns()
    # Exit 1 (a rejection) is a legitimate verdict, not a bench failure; exit 2
    # (broken file / usage) is fatal.
    if proc.returncode == 2:
        sys.exit(f"{label}: oxilean-verify reported a broken input:\n{proc.stderr}")
    report = json.loads(proc.stdout)
    t = report["totals"]
    decls = t["verified"] + t["unsupported"] + t["rejected"]
    walls.append(t1 - t0)

walls.sort()
median_ns = walls[len(walls) // 2]
median_s = median_ns / 1e9
dps = decls / median_s if median_s > 0 else float("inf")
print(f"  {label:24s} {decls:7d} decls  median {median_s*1000:8.1f} ms  {dps:12.0f} decls/sec")

if md_out:
    with open(md_out, "a", encoding="utf-8") as fh:
        fh.write(f"| `{label}` | {decls} | {median_s*1000:.1f} | {dps:,.0f} |\n")
PY
}

echo "=================================================================="
echo " oxilean-verify throughput bench"
echo "=================================================================="
echo " binary : $BIN"
echo " version: $("$BIN" --version 2>/dev/null | head -1)"
echo " CPU    : $CPU ($CORES cores)"
echo " memory : ${MEM_GB} GiB"
echo " OS     : $OS"
echo " rustc  : $RUSTC"
echo " runs   : $RUNS (median reported)"
echo "------------------------------------------------------------------"

if [[ -n "$MD_OUT" ]]; then
    {
        echo "# oxilean-verify throughput"
        echo ""
        echo "- binary: \`$BIN\`"
        echo "- version: $("$BIN" --version 2>/dev/null | head -1)"
        echo "- CPU: $CPU ($CORES cores)"
        echo "- memory: ${MEM_GB} GiB"
        echo "- OS: $OS"
        echo "- rustc: $RUSTC"
        echo "- runs per input: $RUNS (median)"
        echo ""
        echo "| input | decls | median ms | decls/sec |"
        echo "|---|---|---|---|"
    } > "$MD_OUT"
fi

echo " (a) six committed fixtures:"
for f in Nat.add_succ Parity.isEven Tree_Forest Tree_Forest_full point_swap_swap simple_add; do
    bench_one "$f" "$FIXTURE_DIR/$f.ndjson"
done

echo " (b) Init 2,000-declaration slice:"
bench_one "init-2000-slice" "$SLICE"

echo "------------------------------------------------------------------"
echo " done."
if [[ -n "$MD_OUT" ]]; then
    echo " markdown table written to $MD_OUT"
fi
