#!/usr/bin/env bash
set -e

err() {
    echo -e "\e[31m\e[1merror:\e[0m $@" 1>&2;
}

status() {
    WIDTH=12
    printf "\e[32m\e[1m%${WIDTH}s\e[0m %s\n" "$1" "$2"
}

TAG=$1

if [ -z "$TAG" ]; then
    err "Expected tag to be set"
    exit 1
fi

# strip preceding 'v' if it exists on tag
TAG=${TAG/#v}

# Extract crate name and version from tag (format: {crate-name}-{version})
# Example: forc-wallet-0.16.0 -> crate=forc-wallet, version=0.16.0
CRATE_NAME=$(echo "$TAG" | sed -E 's/-[0-9]+\.[0-9]+\.[0-9]+.*//')
VERSION=$(echo "$TAG" | sed -E 's/^.*-([0-9]+\.[0-9]+\.[0-9]+.*)/\1/')

if [ -z "$CRATE_NAME" ] || [ -z "$VERSION" ]; then
    err "Failed to parse crate name and version from tag: $TAG"
    err "Expected format: {crate-name}-{version} (e.g., forc-wallet-0.16.0)"
    exit 1
fi

status "Parsed tag" "$TAG -> crate: $CRATE_NAME, version: $VERSION"

# Find the Cargo.toml for this crate
MANIFEST="${CRATE_NAME}/Cargo.toml"

if [ ! -f "$MANIFEST" ]; then
    err "Manifest not found at $MANIFEST"
    err "Available workspace members:"
    ls -d */ | grep -v target | sed 's|/||'
    exit 1
fi

status "Found manifest" "$MANIFEST"

# Verify the version in Cargo.toml matches the tag
TOML_VERSION=$(cat "$MANIFEST" | dasel -r toml 'package.version')

if [ "$TOML_VERSION" != "$VERSION" ]; then
    err "Crate version $TOML_VERSION in $MANIFEST doesn't match tag version $VERSION"
    exit 1
else
  status "Verified" "Crate version matches tag: $VERSION"
fi
