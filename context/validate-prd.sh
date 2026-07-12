#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
prd="$root/prd.md"

test -s "$prd"
grep -Fxq '# Mail for Agents' "$prd"
grep -Fxq '## Product Requirements Document' "$prd"
grep -Eq '^\*\*Document version:\*\* ' "$prd"
grep -Fxq '## 1. Executive summary' "$prd"
grep -Fxq '## 42. Open questions requiring founder decisions' "$prd"
grep -Fxq '## Final product statement' "$prd"

for section in $(seq 1 42); do
  count="$(grep -Ec "^## ${section}\. " "$prd")"
  if [[ "$count" -ne 1 ]]; then
    echo "Expected exactly one level-two section ${section}; found ${count}." >&2
    exit 1
  fi
done

if grep -En '^(<<<<<<<|=======|>>>>>>>)' "$prd"; then
  echo 'Merge-conflict marker found in prd.md.' >&2
  exit 1
fi

echo "PRD validation passed: $(wc -l < "$prd") lines, sections 1-42 present once."

