#!/usr/bin/env bash
# size-gate.sh — G6: enforce the WASM gzip size budget (brief §8.2).
#
# The "Kernel in a Tab" wasm must stay small: a zero-dependency kernel should be
# small, and if it isn't, something else linked in. Budget: <= 400 KB gzip.
#
# Usage:
#   size-gate.sh <path-to-wasm-file> [budget-kb]
#
# Arguments:
#   <path-to-wasm-file>  Path to the .wasm file to measure.
#   [budget-kb]          Budget in KB of gzipped bytes (default: 400).
#
# Exit codes:
#   0  gzipped size is within budget.
#   1  gzipped size exceeds budget, or an error occurred.
set -euo pipefail

WASM_FILE="${1:-}"
BUDGET_KB="${2:-400}"

if [ -z "$WASM_FILE" ] || [ ! -f "$WASM_FILE" ]; then
    echo "ERROR: wasm file not found: '$WASM_FILE'" >&2
    echo "Usage: $0 <path-to-wasm-file> [budget-kb]" >&2
    exit 1
fi
if ! [[ "$BUDGET_KB" =~ ^[0-9]+$ ]]; then
    echo "ERROR: budget-kb must be a non-negative integer, got: $BUDGET_KB" >&2
    exit 1
fi

RAW_BYTES="$(stat -c%s "$WASM_FILE")"
GZ_BYTES="$(gzip -9 -c "$WASM_FILE" | wc -c)"
BUDGET_BYTES=$(( BUDGET_KB * 1024 ))
GZ_KB=$(( (GZ_BYTES + 512) / 1024 ))

echo "WASM size gate:"
echo "  File:        $WASM_FILE"
echo "  Raw bytes:   $RAW_BYTES"
echo "  Gzip bytes:  $GZ_BYTES  (~${GZ_KB} KB)"
echo "  Budget:      ${BUDGET_KB} KB (${BUDGET_BYTES} bytes) gzip"

if [ "$GZ_BYTES" -gt "$BUDGET_BYTES" ]; then
    echo ""
    echo "FAIL: gzipped wasm is $GZ_BYTES bytes, over the ${BUDGET_BYTES}-byte budget."
    echo "Do NOT strip functionality to hit the number. Investigate what linked in:"
    echo "  wasm-tools print <wasm> | grep -c '(func'   # function count"
    echo "  twiggy top <wasm>                           # top code-size offenders"
    exit 1
fi

echo "  Result:      PASS (within budget)"
exit 0
