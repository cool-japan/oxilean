#!/usr/bin/env bash
# gate-demo-smoke.sh — G8 (partial): prove the "Kernel in a Tab" demo serves
# under a plain static server.
#
# Starts `python3 -m http.server` on a random free port over web/verify-demo,
# curls the three load-bearing assets (index.html, the module script main.js,
# and the .wasm), asserts each returns HTTP 200 and non-empty bytes matching the
# on-disk size, then kills the server. No COOP/COEP, no SharedArrayBuffer — the
# whole point of the demo (brief §6, §8.3).
#
# This does NOT drive a headless browser (no node/headless Chrome in this env);
# it validates the static-serving contract only. The wasm-engine parity itself
# is validated natively (see scripts/gate-determinism.sh) and structurally
# (wasm-tools validate + kernel-symbol presence in the .wasm).
#
# Usage:  bash scripts/gate-demo-smoke.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEMO_DIR="$ROOT/web/verify-demo"

if [ ! -f "$DEMO_DIR/index.html" ]; then
    echo "FAIL: $DEMO_DIR/index.html missing — build the demo first (web/verify-demo/build.sh)." >&2
    exit 1
fi
if [ ! -f "$DEMO_DIR/pkg/oxilean_verify_bg.wasm" ]; then
    echo "FAIL: demo pkg/oxilean_verify_bg.wasm missing — run web/verify-demo/build.sh." >&2
    exit 1
fi

if ! command -v python3 &>/dev/null; then
    echo "FAIL: python3 not found — the demo is required to serve under python3 -m http.server." >&2
    exit 1
fi
if ! command -v curl &>/dev/null; then
    echo "FAIL: curl not found — needed to smoke the served assets." >&2
    exit 1
fi

# Pick a random high port and start the server in the demo directory.
PORT=$(( (RANDOM % 20000) + 20000 ))
echo "--- Serving $DEMO_DIR on 127.0.0.1:$PORT (python3 -m http.server) ---"

( cd "$DEMO_DIR" && python3 -m http.server "$PORT" --bind 127.0.0.1 ) >/dev/null 2>&1 &
SERVER_PID=$!

cleanup() {
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
}
trap cleanup EXIT

# Wait for the server to come up (up to ~5s).
BASE="http://127.0.0.1:$PORT"
UP=0
for _ in $(seq 1 50); do
    if curl -fsS -o /dev/null "$BASE/index.html" 2>/dev/null; then
        UP=1
        break
    fi
    sleep 0.1
done
if [ "$UP" -ne 1 ]; then
    echo "FAIL: server did not come up on $BASE" >&2
    exit 1
fi

FAILED=0

check_asset() {
    local rel="$1"
    local disk="$DEMO_DIR/$rel"
    local url="$BASE/$rel"

    if [ ! -f "$disk" ]; then
        echo "FAIL: $rel is not present on disk ($disk)"
        FAILED=1
        return
    fi

    local code
    code=$(curl -fsS -o /dev/null -w '%{http_code}' "$url" 2>/dev/null || echo "000")
    if [ "$code" != "200" ]; then
        echo "FAIL: $rel returned HTTP $code (expected 200)"
        FAILED=1
        return
    fi

    local served_bytes disk_bytes
    served_bytes=$(curl -fsS "$url" 2>/dev/null | wc -c)
    disk_bytes=$(stat -c%s "$disk")

    if [ "$served_bytes" -eq 0 ]; then
        echo "FAIL: $rel served 0 bytes (empty artifact)"
        FAILED=1
        return
    fi
    if [ "$served_bytes" -ne "$disk_bytes" ]; then
        echo "FAIL: $rel served $served_bytes bytes but on-disk is $disk_bytes"
        FAILED=1
        return
    fi

    echo "OK:   $rel  ->  HTTP 200, $served_bytes bytes"
}

check_asset "index.html"
check_asset "main.js"
check_asset "style.css"
check_asset "pkg/oxilean_verify.js"
check_asset "pkg/oxilean_verify_bg.wasm"

if [ "$FAILED" -ne 0 ]; then
    echo "" >&2
    echo "Demo smoke test FAILED." >&2
    exit 1
fi

echo ""
echo "Demo smoke test PASSED — all assets serve under python3 -m http.server."
exit 0
