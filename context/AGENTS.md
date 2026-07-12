# Mail for Agents planning workspace

This directory holds the operating context for iterative work on `../prd.md`.

## Scope

- Read the whole PRD, this file, `LOOP.md`, and `iterations.md` before editing.
- The PRD improvement loop is explicitly authorized to modify only `../prd.md` and files in this directory.
- Do not create application code or modify sibling repositories from this loop.
- Preserve useful product decisions unless evidence or internal consistency requires a change.

## Working rules

1. Make one coherent, reviewable improvement per iteration.
2. Prefer resolving contradictions, closing a consequential gap, or making requirements testable over adding volume.
3. Check related sections before changing a requirement; update every affected section in the same pass.
4. Do not invent market facts, benchmarks, citations, or user evidence. Mark unknowns as assumptions or founder decisions.
5. Keep the existing voice, heading hierarchy, terminology, and numbered-section structure.
6. Keep `Document version`, `Status`, and `Date` honest when the document materially changes.
7. The files named under **Locked controls** are evaluator infrastructure. Never edit them during an iteration.
8. Run `context/validate-prd.sh` after editing.
9. Do not declare the candidate accepted and do not edit the ledgers. The runner evaluates, keeps or restores, and logs the attempt.
10. Stop after the bounded improvement. Do not begin a second objective in the same iteration.

## Locked controls

- `AGENTS.md`
- `LOOP.md`
- `iteration-prompt.md`
- `evaluator-prompt.md`
- `evaluator-schema.json`
- `validate-prd.sh`
- `run-prd-loop.sh`

## Definition of done for an iteration

- The chosen issue is materially improved in `prd.md`.
- Cross-references and related requirements remain consistent.
- No placeholder citations or unsupported claims were introduced.
- Validation passes.
- The agent handoff states the hypothesis, change, expected improvement, and remaining risks so the runner can preserve it as evidence.
