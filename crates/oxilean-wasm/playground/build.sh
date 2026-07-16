#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WASM_DIR="$(dirname "$SCRIPT_DIR")"

cd "$WASM_DIR"

echo "=== Building OxiLean WASM for playground ==="

# ── 1. wasm-pack build (web target) ─────────────────────────────────────────
# We use --target web so the output is a plain ES module with a default init()
# export — no bundler required.  The web target resolves the .wasm file via
# import.meta.url, which works correctly when served over HTTP.
if command -v wasm-pack &>/dev/null; then
  echo "--- wasm-pack build (web target) ---"
  wasm-pack build \
    --target web \
    --features wasm \
    --out-dir pkg-web \
    --release
else
  echo "WARNING: wasm-pack not found."
  echo "  Install: cargo install wasm-pack"
  echo "  Continuing with existing pkg-web/ if present..."
  if [ ! -d pkg-web ]; then
    echo "ERROR: pkg-web/ does not exist and wasm-pack is unavailable."
    exit 1
  fi
fi

# ── 2. Assemble dist/ ────────────────────────────────────────────────────────
echo "--- Assembling dist/ ---"
mkdir -p dist

# Playground static files
cp playground/index.html dist/
cp playground/style.css  dist/
cp playground/main.js    dist/

# WASM build artefacts (web target)
if [ -d pkg-web ]; then
  # Copy .wasm binary
  find pkg-web -maxdepth 1 -name "*.wasm" -exec cp {} dist/ \;
  # Copy JS bindings (both the main module and the bg glue)
  find pkg-web -maxdepth 1 -name "*.js"   -exec cp {} dist/ \;
  # Copy TypeScript declarations if present (optional, for IDE support)
  find pkg-web -maxdepth 1 -name "*.d.ts" -exec cp {} dist/ \; 2>/dev/null || true
else
  echo "ERROR: pkg-web/ not found after build step."
  exit 1
fi

# ── 3. Verify output ─────────────────────────────────────────────────────────
echo "--- Verifying dist/ ---"
missing=0

for f in index.html style.css main.js; do
  if [ ! -f "dist/$f" ]; then
    echo "  MISSING: dist/$f"
    missing=1
  fi
done

if ! find dist -maxdepth 1 -name "*.wasm" | grep -q .; then
  echo "  MISSING: dist/*.wasm"
  missing=1
fi

if ! find dist -maxdepth 1 -name "oxilean_wasm.js" | grep -q .; then
  echo "  MISSING: dist/oxilean_wasm.js"
  missing=1
fi

if [ "$missing" -ne 0 ]; then
  echo "ERROR: One or more expected files are missing from dist/."
  exit 1
fi

echo ""
echo "=== Playground build complete ==="
echo "  Output: $WASM_DIR/dist/"
echo ""
echo "Serve locally (required — file:// does not load WASM):"
echo "  cd $WASM_DIR/dist && python3 -m http.server 8080"
echo "  Then open: http://localhost:8080"
