#!/usr/bin/env bash
# gate-allow-list.sh — validate the runtime dependency closure of the
# oxilean-verify product (oxilean-kernel + oxilean-export + oxilean-verify)
# against verify/allowed-deps.txt, and assert the allow-list file itself is
# present.
#
# Usage:
#   bash scripts/gate-allow-list.sh [<crate-name> ...]
#
# With no arguments the full product closure is checked:
#   oxilean-kernel oxilean-export oxilean-verify
#
# For every crate the FULL TRANSITIVE runtime dep tree
# (`cargo tree --edges normal`, which excludes dev/build deps) is compared
# against the allow-list: every crate name appearing anywhere in the tree —
# including the root crate itself — must be listed in
# verify/allowed-deps.txt.  A single crates.io dependency sneaking in at any
# depth fails the gate.
#
# Run locally from the workspace root:
#   bash scripts/gate-allow-list.sh
set -euo pipefail

ALLOW_LIST="verify/allowed-deps.txt"

# The product dependency closure (G3). Override by passing crate names.
CRATES=("$@")
if [ "${#CRATES[@]}" -eq 0 ]; then
    CRATES=(oxilean-kernel oxilean-export oxilean-verify)
fi

# ── 1. Existence check ────────────────────────────────────────────────────────
echo "--- Checking allow-list file exists: ${ALLOW_LIST} ---"
if [ ! -f "${ALLOW_LIST}" ]; then
    echo "FAIL: ${ALLOW_LIST} is missing.  Create it before merging." >&2
    exit 1
fi
echo "OK: ${ALLOW_LIST} exists."

# ── 2. Parse the allow-list (ignore comments and blank lines) ─────────────────
ALLOWED=$(grep -v '^\s*#' "${ALLOW_LIST}" | grep -v '^\s*$' | awk '{print $1}' | sort)

if [ -z "${ALLOWED}" ]; then
    echo "FAIL: ${ALLOW_LIST} is empty (after stripping comments)." >&2
    exit 1
fi

echo "Allowed crates:"
echo "${ALLOWED}" | sed 's/^/  /'

# ── 3. Validate each crate's FULL transitive runtime dep tree ────────────────
FAILED=0
for CRATE in "${CRATES[@]}"; do
    echo "--- Validating ${CRATE} runtime dep closure against allow-list ---"

    DEPS=$(cargo tree -p "${CRATE}" --edges normal --prefix none 2>/dev/null \
        | grep -v '^\[' \
        | grep -v '^\s*$' \
        | awk '{print $1}' \
        | sort -u)

    if [ -z "${DEPS}" ]; then
        echo "FAIL: could not resolve the dep tree for '${CRATE}' (is it a workspace member?)"
        FAILED=1
        continue
    fi

    echo "${CRATE} runtime dep tree (crate names, incl. the root):"
    echo "${DEPS}" | sed 's/^/  /'

    while IFS= read -r dep; do
        if ! echo "${ALLOWED}" | grep -qx "${dep}"; then
            echo "FAIL: '${dep}' is in ${CRATE}'s runtime tree but NOT in ${ALLOW_LIST}"
            FAILED=1
        fi
    done <<< "${DEPS}"
done

if [ "${FAILED}" -ne 0 ]; then
    exit 1
fi

echo "OK: the full runtime dependency closure of [${CRATES[*]}] is in the allow-list."
echo "All allow-list checks passed."
