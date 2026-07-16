#!/usr/bin/env bash
# gate-zero-deps.sh — assert that a crate has ZERO external runtime dependencies.
#
# Usage:
#   scripts/gate-zero-deps.sh <crate-name> [<crate-name> ...]
#
# The check uses `cargo tree --edges normal` (runtime deps only, no dev/build)
# and fails if any dependency line appears that is not a WORKSPACE-INTERNAL
# path dependency.  Workspace-internal crates (rendered by cargo tree with a
# local path in parentheses) are allowed — the TCB is the closed set of
# workspace crates passed to this gate, e.g.:
#
#   scripts/gate-zero-deps.sh oxilean-kernel oxilean-export
#
# where oxilean-export depends only on oxilean-kernel (internal, allowed) and
# oxilean-kernel depends on nothing.  Any crates.io dependency anywhere in the
# normal-edge tree fails the gate.
#
# Run locally from the workspace root:
#   bash scripts/gate-zero-deps.sh oxilean-kernel
set -euo pipefail

if [ "$#" -eq 0 ]; then
    echo "Usage: $0 <crate-name> [<crate-name> ...]" >&2
    exit 1
fi

FAILED=0

for CRATE in "$@"; do
    echo "--- Checking zero-dep invariant for: ${CRATE} ---"

    # Capture stdout only; stderr carries progress/lock messages that are not deps.
    OUTPUT=$(cargo tree -p "${CRATE}" --edges normal 2>/dev/null)

    # The first line is the crate itself (e.g. "oxilean-kernel v0.1.2 (path)").
    # Any subsequent non-empty line that is NOT a metadata/lock line means there
    # is a dependency.  We filter:
    #   - The crate header line itself  (starts with the crate name)
    #   - Lines produced by the package resolver that are merely informational
    #     and start with '[' (e.g. "[activated features]" or lock messages)
    #   - Workspace-internal path dependencies: cargo tree renders these with
    #     a filesystem path in parentheses, e.g.
    #         └── oxilean-kernel v0.1.2 (/path/to/repo/crates/oxilean-kernel)
    #     while crates.io deps have no such suffix (or "(*)" for dedup marks,
    #     which we must still catch when they refer to external crates — a
    #     dedup mark only appears for a crate already printed, so filtering by
    #     the path suffix alone is sound for internal crates).
    EXTRA=$(echo "${OUTPUT}" \
        | tail -n +2 \
        | grep -v '^\[' \
        | grep -v '^[[:space:]]*$' \
        | grep -vE '\(/[^)]*\)( \(\*\))?$' \
        || true)

    if [ -n "${EXTRA}" ]; then
        echo "FAIL: ${CRATE} has unexpected external runtime dependencies:"
        echo "${OUTPUT}"
        FAILED=1
    else
        echo "OK: ${CRATE} has zero external runtime dependencies (workspace-internal deps allowed)."
    fi
done

if [ "${FAILED}" -ne 0 ]; then
    exit 1
fi
echo "All zero-dep checks passed."
