# 0122 — engineering posture lives in the harness

- **Date:** 2026-11-27 · **Status:** accepted

## Context

The corpus carries intent and the model's shape. `process/engineering.md` and `process/architecture.md` held how code enters the repository instead, and reached build only through an entry's `per`. Plan filed 62 of 366 entries as "One job, one home" cleanups of duplication that landed while the page already stated the rule, and 8 carried an `interface`. A build agent met the posture only when an entry cited it.

## Decision

- **Posture is harness, read at write time.** Both pages become the engineering playbook in `.temper/`. A path-triggered rule carries each invariant as a standing fact. It loads when a file under `src/`, `tests/`, `benches/` or `sdk/` is read, and that trigger suffices: a build tick reads before it edits, and a plan or sweep tick that only searches gets the topics as posture pages instead. A skill routes to one topic per section, verbatim.
- **The topics are the sweep's posture pages.** The sweep cites them through flume's `citeLocus` (flume#29), so a phrase change files at first noting while ambient shape keeps its bar.
- **One home.** The run-owner invariant folds into the cost topic, and the spec copies are deleted, not kept beside it.

## Rejected

- **Widening `specLocus` over the harness**, as flume does for itself. That makes every rule a derive input, conduct rules included, and spec stops being intent alone.
- **One unconditional rule holding both pages.** About 290 lines in every context (0112); the topics load on demand.
- **Keeping the spec pages beside the harness copy.** Two homes for one fact drift, and the sweep would judge against both.

## Consequences

`spec-system.md` stays, as the corpus's own form. Comments in `src/` and `tests/` that cite `engineering.md` reach the sweep as lines the locus no longer states.
