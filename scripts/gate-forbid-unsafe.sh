#!/usr/bin/env bash
# gate-forbid-unsafe.sh — assert that TCB crates carry #![forbid(unsafe_code)].
#
# Usage:
#   scripts/gate-forbid-unsafe.sh <crate-path-relative-to-workspace-root> [...]
#
# Arguments are crate names as they appear in the workspace (without the
# "crates/" prefix).  The script resolves them to
# crates/<name>/src/lib.rs and greps for the attribute.
#
# Examples:
#   # Check kernel only (current TCB):
#   bash scripts/gate-forbid-unsafe.sh oxilean-kernel
#
#   # Check both kernel and export reader (wave-3 TCB):
#   bash scripts/gate-forbid-unsafe.sh oxilean-kernel oxilean-export
#
# Run locally from the workspace root:
#   bash scripts/gate-forbid-unsafe.sh oxilean-kernel
set -euo pipefail

if [ "$#" -eq 0 ]; then
    echo "Usage: $0 <crate-name> [<crate-name> ...]" >&2
    exit 1
fi

FAILED=0

for CRATE in "$@"; do
    LIB="crates/${CRATE}/src/lib.rs"
    echo "--- Checking #![forbid(unsafe_code)] in: ${LIB} ---"

    if [ ! -f "${LIB}" ]; then
        echo "FAIL: ${LIB} does not exist — cannot verify forbid(unsafe_code)"
        FAILED=1
        continue
    fi

    if grep -qF '#![forbid(unsafe_code)]' "${LIB}"; then
        echo "OK: #![forbid(unsafe_code)] present in ${LIB}"
    else
        echo "FAIL: #![forbid(unsafe_code)] is MISSING from ${LIB}"
        FAILED=1
    fi
done

if [ "${FAILED}" -ne 0 ]; then
    exit 1
fi
echo "All forbid-unsafe checks passed."
