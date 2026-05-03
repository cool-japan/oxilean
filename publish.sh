#!/usr/bin/env bash
set -euo pipefail

# Publish script for oxilean crates
# Copyright: COOLJAPAN OU (Team Kitasan)
# Default: dry-run mode. Pass --publish for real publish.

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

info()    { echo -e "${CYAN}[INFO]${NC} $*"; }
success() { echo -e "${GREEN}[OK]${NC} $*"; }
warn()    { echo -e "${YELLOW}[WARN]${NC} $*"; }
error()   { echo -e "${RED}[ERROR]${NC} $*"; }

# Parse arguments
DRY_RUN=true
for arg in "$@"; do
    case "$arg" in
        --publish)
            DRY_RUN=false
            ;;
        --help|-h)
            echo "Usage: $0 [--publish]"
            echo ""
            echo "  Default:    dry-run mode (no actual publish)"
            echo "  --publish:  real publish to crates.io"
            exit 0
            ;;
        *)
            error "Unknown argument: $arg"
            echo "Usage: $0 [--publish]"
            exit 1
            ;;
    esac
done

# Change to workspace root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Parse workspace version from root Cargo.toml
WORKSPACE_VERSION=""
if [[ -f Cargo.toml ]]; then
    WORKSPACE_VERSION="$(sed -n '/\[workspace\.package\]/,/\[/{ s/^version *= *"\(.*\)"/\1/p; }' Cargo.toml)"
fi

if [[ -z "$WORKSPACE_VERSION" ]]; then
    error "Could not parse workspace version from Cargo.toml"
    exit 1
fi

# Check workspace cleanliness
if ! git diff --quiet 2>/dev/null || ! git diff --cached --quiet 2>/dev/null; then
    if $DRY_RUN; then
        warn "Workspace has uncommitted changes (dry-run: continuing anyway)"
    else
        error "Workspace has uncommitted changes. Please commit or stash before publishing."
        exit 1
    fi
else
    success "Workspace is clean"
fi

echo ""
echo -e "${BOLD}========================================${NC}"
echo -e "${BOLD}  Oxilean Publish Script${NC}"
echo -e "${BOLD}========================================${NC}"
echo ""
echo -e "  Version:  ${GREEN}${WORKSPACE_VERSION}${NC}"
if $DRY_RUN; then
    echo -e "  Mode:     ${YELLOW}DRY-RUN${NC} (pass --publish for real)"
else
    echo -e "  Mode:     ${RED}REAL PUBLISH${NC}"
fi
echo ""

# Tier definitions (dependency order)
TIER1_CRATES=(oxilean-kernel)
TIER2_CRATES=(oxilean-parse oxilean-meta oxilean-std oxilean-codegen oxilean-runtime)
TIER3_CRATES=(oxilean-elab oxilean-build oxilean-lint)
TIER4_CRATES=(oxilean-cli oxilean-wasm)
TIER5_CRATES=(oxilean)

ALL_TIERS=(
    "1:${TIER1_CRATES[*]}"
    "2:${TIER2_CRATES[*]}"
    "3:${TIER3_CRATES[*]}"
    "4:${TIER4_CRATES[*]}"
    "5:${TIER5_CRATES[*]}"
)

# Counters
TOTAL=0
PASSED=0
FAILED=0
FAILED_CRATES=()

publish_crate() {
    local crate="$1"
    local dry_flag=""
    if $DRY_RUN; then
        dry_flag="--dry-run"
    fi

    info "Publishing ${BOLD}${crate}${NC} ..."
    TOTAL=$((TOTAL + 1))

    if cargo publish -p "$crate" $dry_flag --allow-dirty 2>&1; then
        success "${crate} published successfully"
        PASSED=$((PASSED + 1))
    else
        error "${crate} failed to publish"
        FAILED=$((FAILED + 1))
        FAILED_CRATES+=("$crate")
        return 1
    fi
}

tier_sleep() {
    local tier_num="$1"
    if ! $DRY_RUN; then
        warn "Waiting 30s for crates.io index propagation after Tier ${tier_num} ..."
        sleep 30
        success "Index propagation wait complete"
    fi
}

# Publish all tiers
LAST_TIER_IDX=$(( ${#ALL_TIERS[@]} - 1 ))

for i in "${!ALL_TIERS[@]}"; do
    entry="${ALL_TIERS[$i]}"
    tier_num="${entry%%:*}"
    crate_list="${entry#*:}"
    read -ra crates <<< "$crate_list"

    echo ""
    echo -e "${BOLD}--- Tier ${tier_num} (${#crates[@]} crate(s)) ---${NC}"

    tier_failed=false
    for crate in "${crates[@]}"; do
        if ! publish_crate "$crate"; then
            tier_failed=true
        fi
    done

    # Sleep between tiers (not after the last tier)
    if [[ "$i" -lt "$LAST_TIER_IDX" ]] && ! $tier_failed; then
        tier_sleep "$tier_num"
    fi
done

# Summary
echo ""
echo -e "${BOLD}========================================${NC}"
echo -e "${BOLD}  Publish Summary${NC}"
echo -e "${BOLD}========================================${NC}"
echo ""
echo -e "  Version:  ${GREEN}${WORKSPACE_VERSION}${NC}"
if $DRY_RUN; then
    echo -e "  Mode:     ${YELLOW}DRY-RUN${NC}"
else
    echo -e "  Mode:     ${RED}REAL PUBLISH${NC}"
fi
echo -e "  Total:    ${TOTAL}"
echo -e "  Passed:   ${GREEN}${PASSED}${NC}"
echo -e "  Failed:   ${RED}${FAILED}${NC}"

if [[ ${#FAILED_CRATES[@]} -gt 0 ]]; then
    echo ""
    error "Failed crates:"
    for fc in "${FAILED_CRATES[@]}"; do
        echo -e "    ${RED}- ${fc}${NC}"
    done
    echo ""
    exit 1
fi

echo ""
success "All ${TOTAL} crates published successfully!"
echo ""
