#!/usr/bin/env bash
set -euo pipefail

: "${CRATES:?CRATES env is required}"

# Ensure tags are available
git fetch --tags --force || true

# Determine base ref to diff against (previous tag or repo root commit)
if PREV_TAG=$(git describe --tags --abbrev=0 HEAD^ 2>/dev/null); then
  BASE_REF="$PREV_TAG"
else
  BASE_REF=$(git rev-list --max-parents=0 HEAD | tail -n1)
fi
echo "Using BASE_REF=$BASE_REF -> HEAD=${GITHUB_SHA:-HEAD}"

# Build JSON matrix with a changed flag per crate
JSON='{"include":['
SEP=""
ANY_CHANGED=false
while IFS= read -r entry; do
  [ -z "$entry" ] && continue
  name="${entry%%|*}"
  path="${entry##*|}"
  if git diff --quiet "$BASE_REF" "${GITHUB_SHA:-HEAD}" -- "$path"; then
    changed=false
  else
    changed=true
    ANY_CHANGED=true
  fi
  JSON+="$SEP{\"name\":\"$name\",\"path\":\"$path\",\"changed\":$changed}"
  SEP="," 
done <<< "${CRATES}"
JSON+=']}'

echo "matrix=$JSON" >> "$GITHUB_OUTPUT"
echo "any_changed=$ANY_CHANGED" >> "$GITHUB_OUTPUT"
echo "Computed matrix: $JSON"


