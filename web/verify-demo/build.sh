#!/usr/bin/env bash
# build.sh — build the oxilean-verify-wasm artifact and stage the demo.
#
# Rebuilds the wasm (release, target web), copies pkg/ into the demo, refreshes
# the shipped sample export, and writes the measured gzipped-wasm size (in KB)
# into the badge in index.html so "kernel: N KB wasm" is always the real number
# at build/copy time.
#
# Usage:  web/verify-demo/build.sh
set -euo pipefail

# Resolve the workspace root from this script's location.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEMO_DIR="$SCRIPT_DIR"
ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

CRATE="crates/oxilean-verify-wasm"
PKG_OUT="$ROOT/$CRATE/pkg"

echo "==> Building oxilean-verify-wasm (release, target web)..."
( cd "$ROOT" && wasm-pack build "$CRATE" --release --target web \
    --out-dir pkg --out-name oxilean_verify )

# npm packaging groundwork (NOT published here): wasm-pack derives package.json
# from the crate; override only the npm name to the scoped @cooljapan/oxilean-verify.
# description/license/repository/homepage/keywords already come from Cargo.toml.
if [ -f "$PKG_OUT/package.json" ]; then
    python3 - "$PKG_OUT/package.json" <<'PY'
import json, sys
path = sys.argv[1]
with open(path, encoding="utf-8") as f:
    pkg = json.load(f)
pkg["name"] = "@cooljapan/oxilean-verify"
with open(path, "w", encoding="utf-8") as f:
    json.dump(pkg, f, indent=2)
    f.write("\n")
print(f"    npm name set to {pkg['name']}")
PY
fi

echo "==> Staging pkg/ into the demo..."
mkdir -p "$DEMO_DIR/pkg" "$DEMO_DIR/sample"
cp "$PKG_OUT/oxilean_verify.js"            "$DEMO_DIR/pkg/"
cp "$PKG_OUT/oxilean_verify_bg.wasm"       "$DEMO_DIR/pkg/"
cp "$PKG_OUT/oxilean_verify.d.ts"          "$DEMO_DIR/pkg/"
cp "$PKG_OUT/oxilean_verify_bg.wasm.d.ts"  "$DEMO_DIR/pkg/"

echo "==> Refreshing shipped sample export..."
cp "$ROOT/tests/fixtures/lean4export/simple_add.ndjson" "$DEMO_DIR/sample/simple_add.ndjson"

# Measure the gzipped .wasm size in KB and write it into the badge.
GZ_BYTES="$(gzip -9 -c "$DEMO_DIR/pkg/oxilean_verify_bg.wasm" | wc -c)"
KB=$(( (GZ_BYTES + 512) / 1024 ))
RAW_BYTES="$(stat -c%s "$DEMO_DIR/pkg/oxilean_verify_bg.wasm")"

echo "==> Wasm: raw ${RAW_BYTES} B · gzip ${GZ_BYTES} B (~${KB} KB)"

# Replace the <span id="kb">...</span> content with the measured KB.
python3 - "$DEMO_DIR/index.html" "$KB" <<'PY'
import re, sys
path, kb = sys.argv[1], sys.argv[2]
with open(path, encoding="utf-8") as f:
    html = f.read()
html = re.sub(r'(<span id="kb">)\d+(</span>)', rf'\g<1>{kb}\g<2>', html)
with open(path, "w", encoding="utf-8") as f:
    f.write(html)
print(f"    badge updated: kernel: {kb} KB wasm")
PY

echo "==> Done. Serve with:  ( cd web/verify-demo && python3 -m http.server )"
