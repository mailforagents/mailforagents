#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
iterations="${1:-1}"
prd="$root/prd.md"
runs="$root/context/runs"
experiments="$root/context/experiments.jsonl"
ledger="$root/context/iterations.md"
active_baseline=""
active_controls=""

if ! [[ "$iterations" =~ ^[1-9][0-9]*$ ]]; then
  echo 'Usage: context/run-prd-loop.sh [positive-iteration-count]' >&2
  exit 2
fi

for required in codex jq sha256sum; do
  if ! command -v "$required" >/dev/null 2>&1; then
    echo "$required is not available on PATH." >&2
    exit 127
  fi
done

if ! git -C "$root" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo 'This loop requires an initialized Git repository.' >&2
  exit 3
fi
if [[ "$(git -C "$root" branch --show-current)" != "main" ]]; then
  echo 'This loop publishes directly and must be run on the main branch.' >&2
  exit 3
fi
if ! git -C "$root" remote get-url origin >/dev/null 2>&1; then
  echo 'This loop requires an origin remote.' >&2
  exit 3
fi
if [[ -n "$(git -C "$root" status --porcelain)" ]]; then
  echo 'Working tree must be clean before starting the loop.' >&2
  exit 3
fi

locked=(
  "$root/context/AGENTS.md"
  "$root/context/LOOP.md"
  "$root/context/iteration-prompt.md"
  "$root/context/evaluator-prompt.md"
  "$root/context/evaluator-schema.json"
  "$root/context/validate-prd.sh"
  "$root/context/run-prd-loop.sh"
)

restore_on_exit() {
  status=$?
  if [[ -n "$active_baseline" && -f "$active_baseline" ]]; then
    cp "$active_baseline" "$prd"
    echo 'Interrupted or failed attempt: restored its baseline PRD.' >&2
  fi
  if [[ -n "$active_controls" && -d "$active_controls" ]]; then
    for file in "${locked[@]}"; do
      cp "$active_controls/$(basename "$file")" "$file"
    done
    echo 'Interrupted or failed attempt: restored locked controls.' >&2
  fi
  trap - EXIT INT TERM
  exit "$status"
}
trap restore_on_exit EXIT INT TERM

publish_attempt() {
  message="$1"
  git -C "$root" add prd.md context/iterations.md context/experiments.jsonl
  if git -C "$root" diff --cached --quiet; then
    echo 'No tracked loop state changed; nothing to commit.'
    return
  fi
  git -C "$root" commit -m "$message"
  git -C "$root" push origin main
}

mkdir -p "$runs"
touch "$experiments"
"$root/context/validate-prd.sh"

for ((i = 1; i <= iterations; i++)); do
  stamp="$(date -u +%Y%m%dT%H%M%SZ)"
  id="${stamp}-iteration-${i}"
  baseline="$runs/${id}-baseline.md"
  candidate="$runs/${id}-candidate.md"
  handoff="$runs/${id}-agent.md"
  evaluation="$runs/${id}-evaluation.json"
  evaluator_input="$runs/${id}-evaluator-input.md"
  controls_before="$runs/${id}-controls.sha256"
  controls_snapshot="$runs/${id}-locked-controls"

  cp "$prd" "$baseline"
  active_baseline="$baseline"
  mkdir "$controls_snapshot"
  active_controls="$controls_snapshot"
  for file in "${locked[@]}"; do
    cp "$file" "$controls_snapshot/$(basename "$file")"
  done
  baseline_hash="$(sha256sum "$baseline" | cut -d' ' -f1)"
  baseline_lines="$(wc -l < "$baseline")"
  sha256sum "${locked[@]}" > "$controls_before"
  echo "Starting experiment ${i}/${iterations}: $id"

  if ! codex exec \
    --cd "$root" \
    --sandbox workspace-write \
    --skip-git-repo-check \
    --output-last-message "$handoff" \
    - < "$root/context/iteration-prompt.md"; then
    jq -cn --arg id "$id" --arg baseline "$baseline_hash" \
      '{id:$id,result:"crash",baseline_sha256:$baseline,reason:"editing agent failed"}' >> "$experiments"
    printf '\n## %s — reverted (agent failure)\n\n- Baseline restored; see `%s`.\n' "$stamp" "context/runs/$(basename "$handoff")" >> "$ledger"
    cp "$baseline" "$prd"
    active_baseline=""
    active_controls=""
    publish_attempt "prd loop: record failed experiment $id"
    continue
  fi

  cp "$prd" "$candidate"
  candidate_hash="$(sha256sum "$candidate" | cut -d' ' -f1)"
  candidate_lines="$(wc -l < "$candidate")"

  if ! sha256sum --check --status "$controls_before"; then
    jq -cn --arg id "$id" --arg baseline "$baseline_hash" --arg candidate "$candidate_hash" \
      '{id:$id,result:"revert",baseline_sha256:$baseline,candidate_sha256:$candidate,reason:"locked evaluator control changed"}' >> "$experiments"
    printf '\n## %s — reverted (locked control changed)\n\n- Candidate rejected and baseline restored.\n' "$stamp" >> "$ledger"
    cp "$baseline" "$prd"
    for file in "${locked[@]}"; do
      cp "$controls_snapshot/$(basename "$file")" "$file"
    done
    active_baseline=""
    active_controls=""
    publish_attempt "prd loop: reject experiment $id"
    continue
  fi

  if [[ "$candidate_hash" == "$baseline_hash" ]] || ! "$root/context/validate-prd.sh"; then
    reason="structural validation failed"
    [[ "$candidate_hash" == "$baseline_hash" ]] && reason="no PRD change"
    jq -cn --arg id "$id" --arg baseline "$baseline_hash" --arg candidate "$candidate_hash" --arg reason "$reason" \
      '{id:$id,result:"revert",baseline_sha256:$baseline,candidate_sha256:$candidate,reason:$reason}' >> "$experiments"
    printf '\n## %s — reverted (%s)\n\n- Baseline restored; agent handoff: `%s`.\n' "$stamp" "$reason" "context/runs/$(basename "$handoff")" >> "$ledger"
    cp "$baseline" "$prd"
    active_baseline=""
    active_controls=""
    publish_attempt "prd loop: reject experiment $id"
    continue
  fi

  {
    cat "$root/context/evaluator-prompt.md"
    printf '\nBaseline file: `%s`\nCandidate file: `%s`\n' "$baseline" "$candidate"
  } > "$evaluator_input"

  if ! codex exec \
    --cd "$root" \
    --sandbox read-only \
    --skip-git-repo-check \
    --output-schema "$root/context/evaluator-schema.json" \
    --output-last-message "$evaluation" \
    - < "$evaluator_input"; then
    jq -cn --arg id "$id" --arg baseline "$baseline_hash" --arg candidate "$candidate_hash" \
      '{id:$id,result:"revert",baseline_sha256:$baseline,candidate_sha256:$candidate,reason:"independent evaluator failed"}' >> "$experiments"
    printf '\n## %s — reverted (evaluator failure)\n\n- Baseline restored; candidate remains at `%s`.\n' "$stamp" "context/runs/$(basename "$candidate")" >> "$ledger"
    cp "$baseline" "$prd"
    active_baseline=""
    active_controls=""
    publish_attempt "prd loop: record evaluator failure $id"
    continue
  fi

  decision="$(jq -r '.decision' "$evaluation")"
  gates="$(jq -r '.hard_gates_pass' "$evaluation")"
  if [[ "$decision" == "keep" && "$gates" == "true" ]]; then
    result="keep"
    active_baseline=""
    active_controls=""
  else
    result="revert"
    cp "$baseline" "$prd"
    active_baseline=""
    active_controls=""
  fi

  jq -cn \
    --arg id "$id" --arg result "$result" \
    --arg baseline "$baseline_hash" --arg candidate "$candidate_hash" \
    --argjson baseline_lines "$baseline_lines" --argjson candidate_lines "$candidate_lines" \
    --slurpfile evaluation "$evaluation" \
    '{id:$id,result:$result,baseline_sha256:$baseline,candidate_sha256:$candidate,baseline_lines:$baseline_lines,candidate_lines:$candidate_lines,evaluation:$evaluation[0]}' \
    >> "$experiments"

  printf '\n## %s — %s\n\n- Agent handoff: `%s`\n- Independent evaluation: `%s`\n- Candidate artifact: `%s`\n' \
    "$stamp" "$result" "context/runs/$(basename "$handoff")" "context/runs/$(basename "$evaluation")" "context/runs/$(basename "$candidate")" >> "$ledger"
  publish_attempt "prd loop: $result experiment $id"
  echo "Experiment $result; evaluation: $evaluation"
done

trap - EXIT INT TERM
"$root/context/validate-prd.sh"
