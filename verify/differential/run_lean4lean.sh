#!/usr/bin/env bash
# Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
#
# run_lean4lean.sh — drive lean4lean over a Lean module and emit a normalized
# per-declaration verdict file that diff_verdicts.py can join.
#
# lean4lean does NOT read our NDJSON. It checks the declarations of an *olean*
# module on the Lean search path, replaying each constant into a fresh kernel
# environment. Its native behavior with --verbose (see lean4lean/Main.lean):
#
#   * Prints one line per declaration it is about to send to the kernel:
#         adding axiomDecl  <name>
#         adding defnDecl   <name>
#         adding thmDecl    <name>
#         adding opaqueDecl <name>
#         adding inductDecl [<name>, <name>, ...]   (a mutual inductive block)
#         adding quotDecl                            (the Quot primitives block)
#   * Prints "checked N declarations" and exits 0 if every decl passed.
#   * Aborts (nonzero exit) at the FIRST declaration the kernel rejects,
#     printing that declaration's kernel error.
#
# ! IMPORTANT — this runner defaults to --fresh. On this machine, the default
#   (non-fresh) replayFromImports path SEGFAULTS during its region-freeing
#   cleanup (lean_dec_ref_cold in lean4lean_replayFromImports on a worker
#   thread) with the v4.29.0 toolchain — see verify/differential/README.md
#   "lean4lean status". The --fresh path (replayFromFresh) does not free regions
#   and runs cleanly to "checked N declarations". Pass FRESH=0 to force the
#   crashing path for diagnosis.
#
# We map lean4lean's native semantics into the harness verdict vocabulary:
#
#   verified  : an "adding ... <name>" line was printed AND the run either
#               finished (exit 0) or aborted at a strictly later declaration.
#   rejected  : the declaration at the abort point (the kernel error names it).
#   (absent)  : declarations after the abort point are simply not emitted; the
#               diff tool treats a name it never sees as absent.
#
# For an inductDecl block "[A, B]", each of A and B is emitted as a separate
# verified verdict (they are distinct kernel declarations). quotDecl has no
# name and is emitted under the synthetic name "<quot>".
#
# lean4lean has no "unsupported" bucket: it aims to check everything the C++
# kernel does. Constructs it cannot handle surface as an abort, i.e. rejected —
# which is exactly why the diff tool never merges rejected with anything else.
#
# Usage:
#   run_lean4lean.sh <MODULE> <OUT.jsonl>
#     e.g. run_lean4lean.sh Init.Prelude verdicts/lean4lean.Init.Prelude.jsonl
#
# Environment:
#   LEAN4LEAN_DIR   path to the lean4lean checkout (default: ~/work/lean4lean)
#   LEAN4LEAN_BIN   path to the lean4lean binary
#                   (default: $LEAN4LEAN_DIR/.lake/build/bin/lean4lean)
#   FRESH           1 (default) passes --fresh; 0 forces the crashing path.
#
# This runner must be executed from a directory whose Lean search path contains
# MODULE. For Lean core modules, running inside the lean4lean checkout (which
# imports all of core) via `lake env` is sufficient. See the differential README.

set -euo pipefail

if [[ $# -lt 2 ]]; then
    echo "usage: run_lean4lean.sh <MODULE> <OUT.jsonl> [--fresh]" >&2
    exit 2
fi

MODULE="$1"
OUT="$2"
shift 2

# Default to --fresh: the non-fresh path segfaults on this toolchain (see header).
FRESH_FLAG="--fresh"
if [[ "${FRESH:-1}" == "0" ]]; then
    FRESH_FLAG=""
fi
while [[ $# -gt 0 ]]; do
    case "$1" in
        --fresh) FRESH_FLAG="--fresh"; shift ;;
        --no-fresh) FRESH_FLAG=""; shift ;;
        *) echo "unknown argument: $1" >&2; exit 2 ;;
    esac
done

LEAN4LEAN_DIR="${LEAN4LEAN_DIR:-${HOME}/work/lean4lean}"
LEAN4LEAN_BIN="${LEAN4LEAN_BIN:-${LEAN4LEAN_DIR}/.lake/build/bin/lean4lean}"

if [[ ! -x "${LEAN4LEAN_BIN}" ]]; then
    echo "lean4lean binary not found: ${LEAN4LEAN_BIN}" >&2
    echo "build it first: (cd ${LEAN4LEAN_DIR} && lake build lean4lean)" >&2
    exit 2
fi

RAW="$(mktemp)"
trap 'rm -f "${RAW}"' EXIT

echo "[run_lean4lean] lake env ${LEAN4LEAN_BIN} --verbose ${FRESH_FLAG} ${MODULE}" >&2

# lean4lean needs lake's search-path env vars; run it via `lake env` from inside
# the checkout. It aborts (nonzero) at the first rejection — that is a verdict,
# not a script failure, so we capture the exit code instead of dying on it.
set +e
( cd "${LEAN4LEAN_DIR}" && lake env "${LEAN4LEAN_BIN}" --verbose ${FRESH_FLAG} "${MODULE}" ) \
    >"${RAW}" 2>&1
rc=$?
set -e

echo "[run_lean4lean] lean4lean exited ${rc}; parsing ${RAW}" >&2

# Parse the raw output into normalized verdicts.
python3 - "${RAW}" "${OUT}" "${rc}" <<'PY'
import json, re, sys

raw_path, out_path, rc = sys.argv[1], sys.argv[2], int(sys.argv[3])

# "adding <kind> <rest>" — rest is a single name, an inductive-block list
# "[A, B]", or empty (quotDecl).
adding_re = re.compile(r"^adding (\w+)\s*(.*)$")
checked_re = re.compile(r"^checked (\d+) declarations")

# In order of appearance: (name, kind) pairs lean4lean announced. A block of
# mutual inductives contributes several names in one "adding inductDecl" line.
announced = []          # list of (name, kind)
completed_clean = False  # saw "checked N declarations"
with open(raw_path, encoding="utf-8", errors="replace") as fh:
    lines = fh.readlines()

for line in lines:
    line = line.rstrip("\n")
    if checked_re.match(line):
        completed_clean = True
        continue
    m = adding_re.match(line)
    if not m:
        continue
    kind, rest = m.group(1), m.group(2).strip()
    if kind == "quotDecl":
        announced.append(("<quot>", kind))
    elif rest.startswith("[") and rest.endswith("]"):
        inner = rest[1:-1].strip()
        for name in (n.strip() for n in inner.split(",")):
            if name:
                announced.append((name, kind))
    elif rest:
        announced.append((rest, kind))

# Semantics-faithful mapping: with --verbose the LAST announced declaration is
# the one being processed when an abort happens. On clean completion (exit 0
# and/or "checked N") every announced decl verified. On abort every announced
# decl EXCEPT the last verified, and the last is the rejection.
clean = (rc == 0) or completed_clean
error_tail = "" if clean else "".join(lines[-40:]).strip()

records = []
if clean:
    for name, _kind in announced:
        records.append({"name": name, "verdict": "verified", "detail": None})
elif not announced:
    # Aborted before checking any decl (bad module / search-path / import /
    # native crash). This is NOT a per-decl rejection; emit nothing.
    print(
        "[run_lean4lean] aborted before any 'adding' line; this is an "
        "environment/import/native error, not a proof rejection:",
        file=sys.stderr,
    )
    print(error_tail, file=sys.stderr)
else:
    *verified, failing = announced
    for name, _kind in verified:
        records.append({"name": name, "verdict": "verified", "detail": None})
    detail = error_tail.replace("\n", " ")[:400]
    records.append({"name": failing[0], "verdict": "rejected", "detail": detail})

with open(out_path, "w", encoding="utf-8") as out:
    for rec in records:
        out.write(json.dumps(rec, ensure_ascii=False) + "\n")

n_ver = sum(1 for r in records if r["verdict"] == "verified")
n_rej = sum(1 for r in records if r["verdict"] == "rejected")
status = "clean" if clean else f"aborted (exit {rc})"
print(
    f"[run_lean4lean] {n_ver} verified · {n_rej} rejected [{status}] -> {out_path}",
    file=sys.stderr,
)
PY

echo "[run_lean4lean] wrote ${OUT}" >&2
