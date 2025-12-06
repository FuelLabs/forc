#!/usr/bin/env bash
set -e

# Updates releases.toml with dependency information for a released crate.
# Usage: update_releases.sh <crate-name> <version>
#
# This script extracts resolved dependency versions from Cargo.lock and appends
# a new entry to releases.toml for compatibility tracking.

err() {
    echo -e "\e[31m\e[1merror:\e[0m $@" 1>&2;
}

status() {
    WIDTH=12
    printf "\e[32m\e[1m%${WIDTH}s\e[0m %s\n" "$1" "$2"
}

CRATE_NAME=$1
VERSION=$2
RELEASES_FILE="releases.toml"

if [ -z "$CRATE_NAME" ] || [ -z "$VERSION" ]; then
    err "Usage: update_releases.sh <crate-name> <version>"
    exit 1
fi

status "Updating" "$RELEASES_FILE for $CRATE_NAME $VERSION"

DATE=$(date -u +"%Y-%m-%d")
CRATE_TOML="${CRATE_NAME}/Cargo.toml"

# Extract resolved version from Cargo.lock
get_lock_version() {
    local dep_name=$1
    grep -A1 "name = \"$dep_name\"" Cargo.lock 2>/dev/null | \
        grep "version = " | head -1 | \
        sed 's/.*version = "\(.*\)".*/\1/'
}

# Check if crate depends on a package
# Matches both `dep = "version"` and `dep.workspace = true` styles
crate_depends_on() {
    local dep_pattern=$1
    grep -qE "^${dep_pattern}([[:space:]]*=|\.)" "$CRATE_TOML" 2>/dev/null
}

# Only record versions for dependencies the crate actually uses
SWAY_VERSION=""
FUEL_CORE_VERSION=""
FUELS_VERSION=""

if crate_depends_on "sway-core"; then
    SWAY_VERSION=$(get_lock_version "sway-core")
fi

if crate_depends_on "fuel-core"; then
    FUEL_CORE_VERSION=$(get_lock_version "fuel-core")
fi

if crate_depends_on "fuels"; then
    FUELS_VERSION=$(get_lock_version "fuels")
fi

# Create releases.toml if it doesn't exist
if [ ! -f "$RELEASES_FILE" ]; then
    cat > "$RELEASES_FILE" << 'EOF'
# Auto-generated at release time. Do not edit manually.
# This file tracks which dependency versions each release was built against.

EOF
    status "Created" "$RELEASES_FILE"
fi

# Build the TOML entry
{
    echo "[[releases]]"
    echo "crate = \"$CRATE_NAME\""
    echo "version = \"$VERSION\""
    echo "date = \"$DATE\""

    [ -n "$SWAY_VERSION" ] && echo "sway = \"$SWAY_VERSION\""
    [ -n "$FUEL_CORE_VERSION" ] && echo "fuel-core = \"$FUEL_CORE_VERSION\""
    [ -n "$FUELS_VERSION" ] && echo "fuels-rs = \"$FUELS_VERSION\""
    echo ""
} >> "$RELEASES_FILE"

status "Appended" "release entry for $CRATE_NAME $VERSION"
