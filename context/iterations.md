# PRD iteration ledger

This is the durable handoff between Codex runs. Append new entries; do not rewrite history except to correct a factual logging error.

## Baseline — 2026-07-12

- State: `prd.md` is a 3,684-line, build-ready product draft at version 0.1.
- Loop setup: bounded Codex runner, operating prompt, structural validation, and this ledger added.
- Known constraint: the workspace is not currently a Git repository, so review is based on the PRD and this append-only ledger until version control is initialized.
- Suggested first-pass candidates: audit the founder-decision list for MVP blockers; trace MVP scope into acceptance criteria; check workflow execution and SES event handling for contradictory guarantees.

## Loop hardening — 2026-07-12

- Replaced self-certification with a baseline/candidate experiment model.
- Locked the validation and evaluation controls against editing-agent changes.
- Added a fresh read-only semantic evaluator and automatic keep/revert behavior.
- Added append-only machine-readable attempt logging, including rejected and crashed work.
- Human review remains required: the semantic evaluator is comparative and independent, but still model-based.

## Direct publishing — 2026-07-12

- Repository owner authorized bypassing human review for this solo project.
- Every completed experiment now commits its resulting PRD and append-only ledgers to `main` and pushes `origin/main`.
- Local baseline, candidate, and evaluator artifacts remain ignored to avoid repository growth; commit history provides durable rollback.
