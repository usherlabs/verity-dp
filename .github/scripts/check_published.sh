#!/usr/bin/env bash
set -euo pipefail

CRATE_NAME="${1:?crate name is required}"

# Resolve the version using cargo metadata (works with workspace versioning)
TARGET_VERSION=$(cargo metadata --format-version 1 --no-deps \
  | jq -r --arg N "$CRATE_NAME" '.packages[] | select(.name==$N) | .version')

if [ -z "$TARGET_VERSION" ] || [ "$TARGET_VERSION" = "null" ]; then
  echo "Could not resolve version for $CRATE_NAME" >&2
  exit 1
fi

echo "Target version for $CRATE_NAME: $TARGET_VERSION"

# Query crates.io for existing versions
EXISTING=$(curl -fsSL "https://crates.io/api/v1/crates/$CRATE_NAME" \
  | jq -r --arg V "$TARGET_VERSION" '.versions[] | select(.num==$V) | .num' \
  | head -n1 || true)

if [ "$EXISTING" = "$TARGET_VERSION" ]; then
  echo "already=true" >> "$GITHUB_OUTPUT"
  echo "Version $TARGET_VERSION already published for $CRATE_NAME; will skip."
else
  echo "already=false" >> "$GITHUB_OUTPUT"
  echo "Version $TARGET_VERSION not yet published for $CRATE_NAME; will publish."
fi


