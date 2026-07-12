# PRD development loop

The loop turns `prd.md` into a sequence of small, inspectable product decisions rather than an endless rewrite.

## Per-iteration cycle

1. **Baseline:** Validate and snapshot the current PRD; record its hash and size.
2. **Orient:** Read the agent guide, ledger, PRD contents, open questions, risks, roadmap, and acceptance criteria.
3. **Diagnose:** Identify the highest-leverage defect: contradiction, missing decision, untestable requirement, unclear scope, unsafe assumption, or broken traceability.
4. **Hypothesize:** State one expected improvement and choose one bounded objective.
5. **Edit:** Make the smallest complete set of changes, including all affected sections.
6. **Hard gates:** Run the locked structural validator and verify that evaluator controls were untouched.
7. **Independent evaluation:** A fresh, read-only Codex process compares baseline and candidate against the locked rubric.
8. **Keep or revert:** Keep only a candidate that passes every hard gate and earns `keep`; otherwise restore the exact baseline.
9. **Log:** Record every kept, reverted, failed, or crashed attempt in append-only JSONL plus the human ledger.
10. **Publish:** Commit the resulting PRD and ledgers directly to `main`, then push `origin/main`.
11. **Stop:** A new process invocation begins the next experiment.

The editing agent cannot change the acceptance rubric and cannot decide whether its own work is retained. The semantic evaluator is still model-based; commit history is the rollback boundary for any decision the owner later rejects.

The repository owner has explicitly accepted direct-to-`main` automation. The runner therefore refuses to start unless it is on `main`, has an `origin` remote, and begins with no uncommitted tracked or untracked changes outside ignored run artifacts. A failed commit or push stops the loop; it is never reported as published.

## Priority order

1. Contradictions and requirements that could cause incorrect implementation
2. Unresolved founder decisions that block the MVP
3. Security, consent, deliverability, and data-loss risks
4. Missing acceptance criteria and requirement-to-roadmap traceability
5. Architecture or domain-model gaps
6. User-flow ambiguity
7. Language, structure, and concision

## Commands

Run one iteration (recommended):

```bash
context/run-prd-loop.sh
```

Run up to three separate, bounded iterations:

```bash
context/run-prd-loop.sh 3
```

Validate without invoking Codex:

```bash
context/validate-prd.sh
```

The runner deliberately defaults to one iteration. Review `prd.md`, `context/iterations.md`, and `context/experiments.jsonl` between longer runs. Stop immediately with `Ctrl-C`; completed attempts remain logged and an interrupted candidate can be recovered from its baseline artifact.
