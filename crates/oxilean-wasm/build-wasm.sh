#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Building OxiLean WASM ==="

# Clean previous builds
rm -rf pkg pkg-bundler pkg-web pkg-nodejs

# Build for bundler (default/primary)
echo "--- Building for bundler target ---"
wasm-pack build --target bundler --features wasm
mv pkg pkg-bundler

# Build for web target
echo "--- Building for web target ---"
wasm-pack build --target web --features wasm
mv pkg pkg-web

# Build for nodejs target
echo "--- Building for nodejs target ---"
wasm-pack build --target nodejs --features wasm
mv pkg pkg-nodejs

# Use bundler as the primary package
cp -r pkg-bundler pkg

# Patch package.json with @cooljapan/oxilean scope
if command -v node &> /dev/null; then
    node -e "
const fs = require('fs');
const pkg = JSON.parse(fs.readFileSync('pkg/package.json', 'utf8'));
pkg.name = '@cooljapan/oxilean';
pkg.description = 'OxiLean - Lean4-compatible proof assistant for JavaScript/TypeScript';
pkg.keywords = ['lean', 'lean4', 'proof-assistant', 'type-theory', 'wasm', 'webassembly', 'theorem-prover'];
pkg.license = 'Apache-2.0';
pkg.repository = { type: 'git', url: 'https://github.com/cool-japan/oxilean' };
pkg.homepage = 'https://github.com/cool-japan/oxilean';
pkg.bugs = { url: 'https://github.com/cool-japan/oxilean/issues' };
pkg.author = 'COOLJAPAN OU (Team Kitasan)';
fs.writeFileSync('pkg/package.json', JSON.stringify(pkg, null, 2) + '\n');
"
    echo "--- Patched package.json with @cooljapan/oxilean ---"
else
    echo "WARNING: node not found, skipping package.json patching"
fi

echo ""
echo "=== Build complete ==="
echo "  bundler (primary): pkg/"
echo "  web:               pkg-web/"
echo "  nodejs:            pkg-nodejs/"
echo ""
echo "To publish:  cd pkg && npm publish --access public"
