#!/usr/bin/env bash
# export-gate.sh — Verify that a built WASM file has at least MIN_EXPORTS exports.
#
# Usage:
#   export-gate.sh <path-to-wasm-file> [min-exports]
#
# Arguments:
#   <path-to-wasm-file>  Path to the .wasm file to inspect.
#   [min-exports]        Minimum number of exports required (default: 10).
#
# Exit codes:
#   0  Export count meets or exceeds the minimum.
#   1  Export count is below the minimum, or an error occurred.
#
# Tool preference order:
#   1. wasm-tools  (Bytecode Alliance; cargo install wasm-tools)
#   2. wasm-objdump (from wabt; apt install wabt)
#   3. python3 built-in WASM binary parser (zero dependencies)
#
# If none of the above is available, the script exits with an error.

set -euo pipefail

WASM_FILE="${1:-}"
MIN_EXPORTS="${2:-10}"

if [ -z "$WASM_FILE" ]; then
    echo "ERROR: no .wasm file specified." >&2
    echo "Usage: $0 <path-to-wasm-file> [min-exports]" >&2
    exit 1
fi

if [ ! -f "$WASM_FILE" ]; then
    echo "ERROR: file not found: $WASM_FILE" >&2
    exit 1
fi

# Validate MIN_EXPORTS is a positive integer
if ! [[ "$MIN_EXPORTS" =~ ^[0-9]+$ ]]; then
    echo "ERROR: min-exports must be a non-negative integer, got: $MIN_EXPORTS" >&2
    exit 1
fi

count_exports_python() {
    python3 - "$1" <<'PYEOF'
#!/usr/bin/env python3
"""Parse WASM export section without any external dependencies."""
import sys


def read_leb128_u32(data: bytes, pos: int) -> tuple[int, int]:
    """Read an unsigned LEB128-encoded integer. Returns (value, new_pos)."""
    value = 0
    shift = 0
    while True:
        byte = data[pos]
        pos += 1
        value |= (byte & 0x7F) << shift
        if not (byte & 0x80):
            break
        shift += 7
    return value, pos


def count_exports(path: str) -> int:
    with open(path, "rb") as f:
        data = f.read()

    # Verify WASM magic bytes: \0asm
    if data[:4] != b"\x00asm":
        print(f"ERROR: {path!r} is not a valid WASM file (bad magic bytes)", file=sys.stderr)
        sys.exit(1)

    # Skip magic (4 bytes) + version (4 bytes)
    pos = 8
    while pos < len(data):
        section_id = data[pos]
        pos += 1
        # Read section size (LEB128)
        section_size, pos = read_leb128_u32(data, pos)
        section_end = pos + section_size

        if section_id == 7:  # Export section
            # Read export count (LEB128)
            export_count, _ = read_leb128_u32(data, pos)
            return export_count

        pos = section_end

    # No export section found — zero exports
    return 0


if __name__ == "__main__":
    n = count_exports(sys.argv[1])
    print(n)
PYEOF
}

# --- Determine export count using available tooling ---

if command -v wasm-tools &>/dev/null; then
    TOOL="wasm-tools"
    EXPORT_COUNT=$(wasm-tools dump "$WASM_FILE" 2>/dev/null \
        | grep -c "^  Export" || true)
    # wasm-tools dump output format: lines starting with "  Export"
    # Fall back to python if wasm-tools dump doesn't produce expected output
    if [ "$EXPORT_COUNT" -eq 0 ]; then
        # Try wasm-tools json format
        EXPORT_COUNT=$(wasm-tools print --json "$WASM_FILE" 2>/dev/null \
            | python3 -c "import sys,json; d=json.load(sys.stdin); exps=d.get('exports',[]); print(len(exps))" 2>/dev/null || echo "0")
    fi
    if [ "$EXPORT_COUNT" -eq 0 ]; then
        TOOL="python3 (wasm-tools fallback)"
        EXPORT_COUNT=$(count_exports_python "$WASM_FILE")
    fi
elif command -v wasm-objdump &>/dev/null; then
    TOOL="wasm-objdump"
    # wasm-objdump -x prints sections; export entries appear as lines starting with spaces and a dash
    EXPORT_COUNT=$(wasm-objdump -x "$WASM_FILE" 2>/dev/null \
        | grep -c "^ - " || true)
    if [ "$EXPORT_COUNT" -eq 0 ]; then
        TOOL="python3 (wasm-objdump fallback)"
        EXPORT_COUNT=$(count_exports_python "$WASM_FILE")
    fi
elif command -v python3 &>/dev/null; then
    TOOL="python3"
    EXPORT_COUNT=$(count_exports_python "$WASM_FILE")
else
    echo "ERROR: No WASM inspection tool found." >&2
    echo "Install one of:" >&2
    echo "  cargo install wasm-tools   (preferred)" >&2
    echo "  apt install wabt           (provides wasm-objdump)" >&2
    echo "  python3                    (zero-dependency fallback)" >&2
    exit 1
fi

echo "WASM export gate:"
echo "  File:     $WASM_FILE"
echo "  Tool:     $TOOL"
echo "  Exports:  $EXPORT_COUNT"
echo "  Minimum:  $MIN_EXPORTS"

if [ "$EXPORT_COUNT" -lt "$MIN_EXPORTS" ]; then
    echo ""
    echo "FAIL: export count $EXPORT_COUNT is below minimum $MIN_EXPORTS."
    echo ""
    echo "Likely cause: wasm-pack was not invoked with --features wasm."
    echo "The #[wasm_bindgen] items in wasm_api.rs are gated on #[cfg(feature = \"wasm\")]."
    echo "Without that feature, dead-code elimination removes all user-facing exports,"
    echo "leaving only the 'memory' export required by the WASM spec."
    echo ""
    echo "Fix: add --features wasm to every wasm-pack build invocation."
    exit 1
fi

echo "  Result:   PASS"
exit 0
