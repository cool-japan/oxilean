#!/usr/bin/env bash
# Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
#
# regen-corpus.sh — reproduce the oxilean-verify NDJSON corpus from scratch.
#
# This is the V5 re-export pipeline: elan + lean4export -> NDJSON. Every pin is
# hardcoded here so the corpus is byte-reproducible by anyone with the same
# toolchain. The pins below MUST match the ones the reader is built against
# (oxilean_export::LEAN_TOOLCHAIN / LEAN4EXPORT_COMMIT) and the ones recorded in
# every oxilean-verify report.
#
# What it does:
#   1. verifies elan + the pinned Lean toolchain are installed,
#   2. clones/pins lean4export at the exact commit and `lake build`s it,
#   3. re-exports every corpus file into $CORPUS_DIR,
#   4. copies the small fixtures (< 1 MB) into tests/fixtures/lean4export/,
#   5. writes a corpus README with the toolchain versions and per-file repro
#      commands.
#
# It is idempotent: existing clones/builds are reused. Pass --force to rebuild.
#
# Usage:
#   scripts/regen-corpus.sh [--force] [--fixtures-only] [--skip-init]
#
# Environment overrides (defaults shown):
#   WORK_DIR      ~/work                       (parent of the external checkouts)
#   CORPUS_DIR    ~/work/oxilean-corpus        (where NDJSON is written)
#   SCRATCH_DIR   ~/work/lean4-scratch         (custom fixture Lean project)
set -euo pipefail

# --- PINS (hardcoded; keep in lockstep with oxilean-export) ------------------
LEAN_TOOLCHAIN="leanprover/lean4:v4.32.0-rc1"
LEAN_GITHASH="b4812ae53eea93439ad5dce5a5c26591c31cb697"
LEAN4EXPORT_REPO="https://github.com/leanprover/lean4export"
LEAN4EXPORT_COMMIT="3de59f10bc4b4a0f2de698597aeb1246caa0df0a"
NDJSON_FORMAT_VERSION="3.1.0"

WORK_DIR="${WORK_DIR:-$HOME/work}"
CORPUS_DIR="${CORPUS_DIR:-$HOME/work/oxilean-corpus}"
SCRATCH_DIR="${SCRATCH_DIR:-$HOME/work/lean4-scratch}"
LEAN4EXPORT_DIR="$WORK_DIR/lean4export"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FIXTURE_DIR="$ROOT/tests/fixtures/lean4export"

FORCE=0
FIXTURES_ONLY=0
SKIP_INIT=0
while [[ $# -gt 0 ]]; do
    case "$1" in
        --force)          FORCE=1; shift ;;
        --fixtures-only)  FIXTURES_ONLY=1; shift ;;
        --skip-init)      SKIP_INIT=1; shift ;;
        -h|--help)
            sed -n '2,40p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
            exit 0 ;;
        *) echo "unknown argument: $1" >&2; exit 2 ;;
    esac
done

# Ensure elan-provided tools (lake, lean) are reachable.
if [[ -d "$HOME/.elan/bin" ]]; then
    export PATH="$HOME/.elan/bin:$PATH"
fi
command -v lake >/dev/null 2>&1 || {
    echo "FATAL: lake not found on PATH; install elan (~/.elan) first." >&2
    exit 2
}

echo "== regen-corpus =="
echo "  Lean toolchain : $LEAN_TOOLCHAIN ($LEAN_GITHASH)"
echo "  lean4export    : $LEAN4EXPORT_COMMIT"
echo "  NDJSON format  : $NDJSON_FORMAT_VERSION"
echo "  corpus dir     : $CORPUS_DIR"
echo

# --- 1. lean4export checkout + build -----------------------------------------
if [[ ! -d "$LEAN4EXPORT_DIR/.git" ]]; then
    echo "-- cloning lean4export --"
    git clone "$LEAN4EXPORT_REPO" "$LEAN4EXPORT_DIR"
fi
(
    cd "$LEAN4EXPORT_DIR"
    CURRENT="$(git rev-parse HEAD)"
    if [[ "$CURRENT" != "$LEAN4EXPORT_COMMIT" ]]; then
        echo "-- pinning lean4export to $LEAN4EXPORT_COMMIT --"
        git fetch --all --tags
        git checkout "$LEAN4EXPORT_COMMIT"
    fi
    if [[ "$FORCE" == "1" || ! -x ".lake/build/bin/lean4export" ]]; then
        echo "-- lake build lean4export --"
        lake build
    fi
)
EXPORTER="$LEAN4EXPORT_DIR/.lake/build/bin/lean4export"
[[ -x "$EXPORTER" ]] || { echo "FATAL: lean4export binary missing: $EXPORTER" >&2; exit 2; }

mkdir -p "$CORPUS_DIR"

# export_from <project_dir> <out_name> <mod> [decl...]
# Runs lean4export inside <project_dir> for the given module and optional decls.
export_from() {
    local proj="$1" out="$2" mod="$3"; shift 3
    local dest="$CORPUS_DIR/$out"
    echo "-- export $out ($mod ${*:-})"
    if [[ $# -eq 0 ]]; then
        ( cd "$proj" && lake env "$EXPORTER" "$mod" ) > "$dest"
    else
        ( cd "$proj" && lake env "$EXPORTER" "$mod" -- "$@" ) > "$dest"
    fi
}

if [[ "$FIXTURES_ONLY" == "0" ]]; then
    # --- 2. Lean core Init exports (run from the lean4export project) ---------
    if [[ "$SKIP_INIT" == "0" ]]; then
        export_from "$LEAN4EXPORT_DIR" "Init.ndjson"          Init
    fi
    export_from "$LEAN4EXPORT_DIR" "Nat.add_succ.ndjson"  Init Nat.add_succ
    export_from "$LEAN4EXPORT_DIR" "Nat.ndjson"           Init Nat
    export_from "$LEAN4EXPORT_DIR" "List.ndjson"          Init List
    export_from "$LEAN4EXPORT_DIR" "Prod.mk.ndjson"       Init Prod.mk
    export_from "$LEAN4EXPORT_DIR" "Quot.lift.ndjson"     Init Quot.lift

    # --- 3. Custom scratch exports (run from the scratch project) ------------
    if [[ -d "$SCRATCH_DIR" ]]; then
        export_from "$SCRATCH_DIR" "simple_add.ndjson"       Scratch simple_add
        export_from "$SCRATCH_DIR" "point_swap_swap.ndjson"  Scratch point_swap_swap
        export_from "$SCRATCH_DIR" "Parity.isEven.ndjson"    Scratch Parity.isEven
        export_from "$SCRATCH_DIR" "Tree_Forest.ndjson"      Scratch Tree Forest
        export_from "$SCRATCH_DIR" "Tree_Forest_full.ndjson" Scratch Tree Forest Tree.size Forest.size
    else
        echo "WARNING: scratch project $SCRATCH_DIR not found; skipped custom fixtures." >&2
    fi
fi

# --- 4. copy small fixtures into the repo ------------------------------------
echo "-- copying fixtures < 1 MB into $FIXTURE_DIR --"
mkdir -p "$FIXTURE_DIR"
for f in Nat.add_succ point_swap_swap Tree_Forest Tree_Forest_full simple_add Parity.isEven; do
    src="$CORPUS_DIR/$f.ndjson"
    if [[ -f "$src" ]]; then
        sz=$(stat -c%s "$src" 2>/dev/null || echo 0)
        if [[ "$sz" -lt 1048576 ]]; then
            cp "$src" "$FIXTURE_DIR/$f.ndjson"
            echo "   copied $f.ndjson ($sz bytes)"
        else
            echo "   SKIP $f.ndjson (>= 1 MB, kept in corpus only)"
        fi
    fi
done

# --- 5. write the corpus README ----------------------------------------------
echo "-- writing $CORPUS_DIR/README.md --"
cat > "$CORPUS_DIR/README.md" <<EOF
# oxilean-corpus

Lean 4 NDJSON export corpus for the oxilean-verify campaign.
Regenerated by \`scripts/regen-corpus.sh\` in the oxilean repo.

## Toolchain Versions (pinned)

- **Lean toolchain:** $LEAN_TOOLCHAIN (commit $LEAN_GITHASH)
- **lean4export:** commit $LEAN4EXPORT_COMMIT
- **lean4export binary:** $EXPORTER
- **Format version:** NDJSON v$NDJSON_FORMAT_VERSION
- **Generated:** $(date +%Y-%m-%d)

## Repro

\`\`\`
scripts/regen-corpus.sh          # full corpus (Init.ndjson is ~330 MB)
scripts/regen-corpus.sh --skip-init   # everything except the huge Init export
\`\`\`

Init.ndjson is too large to commit; it lives here in $CORPUS_DIR only.
Fixtures under 1 MB are copied into tests/fixtures/lean4export/ in the repo.
EOF

echo
echo "== regen-corpus done =="
ls -la "$CORPUS_DIR"/*.ndjson 2>/dev/null || true
