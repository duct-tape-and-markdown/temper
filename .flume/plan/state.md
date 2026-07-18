# Plan state

- Spec derived through: 4adb1fb
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: provider done — verbs next
- This tick: POSTURE SWEEP — `provider` subsystem (`src/builtin.rs`,
  `src/builtin_kind.rs`, architecture.md's codemap). Inbox and refactor
  captures were empty; spec delta was empty (`git log 4adb1fb..HEAD --
  specs/` empty); the post-ship reconciliation window was empty too (`git
  log 60faee0..HEAD -- src/ sdk/src/ tests/` empty — every commit since
  60faee0 has been a `plan:` tick touching only `.flume/**`). `provider` was
  next in rotation (`judges` swept last tick) and had never been swept this
  cycle, so it ran regardless of the empty window.
  `builtin.rs` (98 lines): clean except `kind_narrowing_clause`
  (50-76), already queued for relocation by the open
  ROSTER-BUILTIN-KIND-NARROWING-RELOCATE entry — not re-reported.
  `builtin_kind.rs` (1130 lines): grep-verified every `pub fn` for outside
  consumers (`definition`, `qualified`, `definitions`, `skill_features`,
  `rule_features`, `features`). Five have real external callers across
  main.rs/install.rs/bundle.rs/import.rs/frontmatter.rs and a dozen
  `tests/*.rs` files. `qualified` (508-522) has none — zero hits for
  `builtin_kind::qualified` or a bare `qualified(` call anywhere in src/,
  tests/, sdk/src/, sdk/test/ outside its own inline test
  (`qualified_names_every_embedded_kind_by_its_own_bare_name`, 758-775).
  Its own doc comment already concedes it as an unadopted convenience
  ("kept for call sites that ask for a kind's identity rather than its full
  definition") — no call site ever took it up. Filed
  BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-PRUNE (engineering.md, "An export
  earns its consumer"), the same precedent as
  ENGINE-SELECTOR-LABEL-ZERO-CONSUMER-PRUNE and
  KIND-ZERO-CONSUMER-EXPORTS-PRUNE. Checked the empty-`Result`-Errr shape
  too (`KindError` is `pub enum KindError {}`, uninhabited, threading a
  vacuous `Result` through `definition`/`qualified`/`definitions`) — this
  is not the DISCOVERY-class residue (a retired second path leaving a
  variant unconstructable): the doc comments on all three fns already
  state the design choice plainly ("kept for API stability"), so it reads
  as a documented judgment call, not an unflagged gap; left alone. No
  cohesion or duplicate-matcher issues found — the module is one job
  (the shipped kinds' definitions and their cited format facts).
- Queue: 19 pending — 8 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-
  COLLAPSE, FRONTMATTER-TEST-SYNTHETIC-KINDS, ROSTER-BUILTIN-KIND-
  NARROWING-RELOCATE, DOCUMENT-RETIRED-FENCE-SURFACE-PRUNE,
  READ-EXPLAIN-STRAND-VISIBILITY-NARROW, ENGINE-SELECTOR-LABEL-ZERO-
  CONSUMER-PRUNE, REPORTER-SEVERITY-WORD-CONSOLIDATE,
  BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-PRUNE; all disjoint files —
  BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-PRUNE shares builtin_kind.rs only
  with the blockedBy'd EXTRACT-FOUNDATION-BOUNDARY-RESTORE, never open
  alongside it), 9 chained blockedBy (DRIFT-LOCK-ROW-WALK-CONSOLIDATION →
  DRIFT-EMIT-LOCK-PARSE-HOIST → PLACEMENT-MODULE-EXTRACTION →
  EXTRACT-FOUNDATION-BOUNDARY-RESTORE → {KIND-ZERO-CONSUMER-EXPORTS-
  PRUNE → CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH →
  CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP, DRIFT-SOURCE-DEP-PARSE-HOIST →
  GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE}), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. No live refactor
  captures; inbox empty.

Plan continues: yes — the posture sweep resumes at its `verbs` leg next
tick (job 4, rotation: `provider` done, `verbs` next).
