#!/usr/bin/env bash
# Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
#
# run_oxilean.sh — drive oxilean-verify over a lean4export NDJSON file and emit
# a normalized per-declaration verdict file that diff_verdicts.py can join.
#
# oxilean-verify already produces exactly the verdict vocabulary the harness
# needs (verified / unsupported / rejected). With --json-full it emits a
# per-declaration `decls` array. We run it and convert that array into a
# .jsonl stream of {"name","verdict","detail"} records.
#
# Usage:
#   run_oxilean.sh <FILE.ndjson> <OUT.jsonl> [--limits corpus|default]
#
# Environment:
#   OXILEAN_VERIFY   path to the oxilean-verify binary
#                    (default: target/release/oxilean-verify under the repo root)
#   LIMITS           default | corpus (default: corpus; whole-corpus exports
#                    such as Init.ndjson legitimately need the large budget)

set -euo pipefail

if [[ $# -lt 2 ]]; then
    echo "usage: run_oxilean.sh <FILE.ndjson> <OUT.jsonl> [--limits corpus|default]" >&2
    exit 2
fi

INPUT="$1"
OUT="$2"
shift 2

LIMITS="${LIMITS:-corpus}"
while [[ $# -gt 0 ]]; do
    case "$1" in
        --limits)
            LIMITS="$2"
            shift 2
            ;;
        *)
            echo "unknown argument: $1" >&2
            exit 2
            ;;
    esac
done

# Locate the repo root (two levels up from verify/differential/).
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
BIN="${OXILEAN_VERIFY:-${REPO_ROOT}/target/release/oxilean-verify}"

if [[ ! -x "${BIN}" ]]; then
    echo "oxilean-verify binary not found or not executable: ${BIN}" >&2
    echo "build it first: cargo build --release -p oxilean-verify" >&2
    exit 2
fi
if [[ ! -f "${INPUT}" ]]; then
    echo "input file not found: ${INPUT}" >&2
    exit 2
fi

REPORT_JSON="$(mktemp)"
trap 'rm -f "${REPORT_JSON}"' EXIT

echo "[run_oxilean] ${BIN} --limits ${LIMITS} --json-full ${INPUT}" >&2

# Do not let a rejection (exit 1) abort the harness: a rejection is a legitimate
# verdict we want to record and diff, not a script failure. Exit 2 (broken file
# / usage) is a real error and is surfaced.
set +e
"${BIN}" --quiet --no-color --limits "${LIMITS}" \
    --json-full --json "${REPORT_JSON}" "${INPUT}" >/dev/null 2>"${REPORT_JSON}.err"
rc=$?
set -e
if [[ ${rc} -eq 2 ]]; then
    echo "[run_oxilean] oxilean-verify reported a broken input (exit 2):" >&2
    cat "${REPORT_JSON}.err" >&2
    rm -f "${REPORT_JSON}.err"
    exit 2
fi
rm -f "${REPORT_JSON}.err"

# Convert the report's `decls` array into a normalized .jsonl verdict stream.
python3 - "${REPORT_JSON}" "${OUT}" <<'PY'
import json, sys
report_path, out_path = sys.argv[1], sys.argv[2]
with open(report_path, encoding="utf-8") as fh:
    report = json.load(fh)
decls = report.get("decls")
if decls is None:
    sys.exit("oxilean-verify report has no `decls` array; run with --json-full")
with open(out_path, "w", encoding="utf-8") as out:
    for d in decls:
        rec = {"name": d["name"], "verdict": d["verdict"], "detail": d.get("detail")}
        out.write(json.dumps(rec, ensure_ascii=False) + "\n")
t = report.get("totals", {})
print(
    "[run_oxilean] {v} verified · {u} unsupported · {r} rejected -> {out}".format(
        v=t.get("verified"), u=t.get("unsupported"), r=t.get("rejected"), out=out_path
    ),
    file=sys.stderr,
)
PY

echo "[run_oxilean] wrote ${OUT}" >&2
