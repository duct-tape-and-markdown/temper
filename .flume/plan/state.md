# Plan state

- **Phase:** reconcile. HEAD 48695de.
- **Last human action:** fork `reachability-gate-mechanism` **RESOLVED** (commit
  48695de, option b) — reachability's opt-in + severity are the **assembly's**
  declaration, like `degree`, never a package clause; `specs/45-governance.md`
  corrected in the same commit ("assembly's declaration", line 141).
- **This tick:** un-parked **REACHABILITY-WIRE** — its sole remaining blocker (that
  fork) is resolved, so gate → `open`, `dependsOnForks` dropped, and files/
  acceptance rewritten for the resolved mechanism: assembly-declared opt-in parsed
  in `src/compose.rs` (like `degree`), wired in `src/main.rs`, the stale
  `src/graph.rs:338-340` doc comment ("the package clause's choice") swept, and the
  severity threaded through the finding — **not** `contract.rs`/`engine.rs` (assembly,
  not package). Re-confirmed on disk: `reachable`/`world`/`dead_activation` at
  graph.rs:342 have no main.rs caller (654-666 = admissibility/check/acyclic/degree);
  built-in kinds' `Activation` reaches the engine via `builtin_kind::definitions()`.
  Carried the other five entries unchanged. Inbox empty; open-questions already
  records the fork RESOLVED.
- **In flight / pickable:** REACHABILITY-WIRE (`open`, sole pickable). MEMORY-KIND,
  AGENT-KIND, EXTRACTION-VOCAB-GAPS, PACKAGING-CHANNELS, COMMUNITY-DOCS remain
  human-gated (curated data / no consumer / release creds / fence-widen).
- **Next:** build picks REACHABILITY-WIRE and ships it one validated commit.

Plan continues: no — REACHABILITY-WIRE is now `open` and pickable; hand to build.
Re-planning an unchanged queue would be the failure mode.
