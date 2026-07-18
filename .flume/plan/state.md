# Plan state

- Spec derived through: 4adb1fb
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: judges done — provider next
- This tick: POSTURE SWEEP. Jobs 1-3 quiet: inbox empty, no live refactor
  captures at tick start; `git log 4adb1fb..HEAD -- specs/` empty (no spec
  delta); `git log 60faee0..HEAD -- src/ sdk/src/ tests/` empty (no
  post-ship window to audit or sweep). Job 4 was live — rotation mid-way
  (`pipeline done — judges next`) makes it live regardless of window
  content. Swept the `judges` subsystem (architecture.md codemap: `engine`,
  `graph`, `dial`, `coverage`, `coverage_note`, `display`, `reporter`),
  delegating the read to a foreground agent over engineering.md's
  postures plus the cohesion/dead-plumbing lenses, then independently
  verifying every candidate before filing:
  - `dial.rs`, `coverage.rs`, `coverage_note.rs`, `display.rs` are clean —
    every `pub`/`pub(crate)` item has a real outside caller, no `_` arm
    over a shared enum, no vacuous judge test.
  - `engine.rs` yielded a verified finding: `Selector::label` (424-433) is
    `pub` but grep-verified zero-consumer outside `finding()` (689), the
    same file — unlike its sibling `Selector::noun` (438-444), whose doc
    comment names the real cross-module consumer (`crate::graph::degree`).
    Filed ENGINE-SELECTOR-LABEL-ZERO-CONSUMER-PRUNE (`engineering.md`, "An
    export earns its consumer") — gate open, disjoint (only src/engine.rs).
  - `reporter.rs` yielded a verified finding: `github` (184-187) and
    `sarif` (219-222) each carry their own identical
    `Severity::Error => "error", Severity::Warn => "warning"` match — the
    same normalizer written twice in one file. Filed
    REPORTER-SEVERITY-WORD-CONSOLIDATE (`engineering.md`, "One job, one
    home") — gate open, disjoint (only src/reporter.rs).
  - `graph.rs` yielded a design-decision-tier finding, not filed directly:
    `resolved_edges`/`resolved_arcs` (958-1010) is independently
    recomputed by up to four call sites inside one `gate()` invocation
    (`check` at 111 does its own from-scratch parallel walk; `acyclic`
    always calls `resolved_arcs`; `degree`/`mention_reachable` call it
    again when opted in) — confirmed on disk via `main.rs:1130-1168`. The
    shared-walk shape isn't mechanical (`check` needs the *dangling* half
    `resolved_edges` discards, so the consolidation has to decide the
    shared output's shape) — captured to
    `.flume/refactor/plan-graph-resolved-edge-walk-duplication.md` per
    the posture-sweep rule's routing split (mechanical → entry,
    design-decision → refactor capture), for next tick's inbox job to
    verify and file. `MAX_IMPORT_HOPS`'s cite (graph.rs:55-59) matches
    the already-parked IMPORT-HOP-CAP-CITE — not re-reported.
  Rotation advances: `judges` done, `provider` next (`builtin`,
  `builtin_kind`).
- Queue: 17 pending — 7 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-
  COLLAPSE, FRONTMATTER-TEST-SYNTHETIC-KINDS, ROSTER-BUILTIN-KIND-
  NARROWING-RELOCATE, DOCUMENT-RETIRED-FENCE-SURFACE-PRUNE,
  READ-EXPLAIN-STRAND-VISIBILITY-NARROW, ENGINE-SELECTOR-LABEL-ZERO-
  CONSUMER-PRUNE, REPORTER-SEVERITY-WORD-CONSOLIDATE; all disjoint
  files), 8 chained blockedBy (DRIFT-LOCK-ROW-WALK-CONSOLIDATION →
  DRIFT-EMIT-LOCK-PARSE-HOIST → PLACEMENT-MODULE-EXTRACTION →
  EXTRACT-FOUNDATION-BOUNDARY-RESTORE → {KIND-ZERO-CONSUMER-EXPORTS-
  PRUNE → CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH →
  CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP, DRIFT-SOURCE-DEP-PARSE-HOIST}), 2
  parked on human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-
  REMAINDER). Open forks: (multi-harness-projection), (lazy-grounds)
  unchanged. 1 live refactor capture filed this tick
  (plan-graph-resolved-edge-walk-duplication.md, for next tick's inbox job
  to drain); inbox empty.

Plan continues: yes — the inbox job drains the freshly-filed refactor
capture next tick (job 1 precedes the posture sweep's `provider` leg in
priority order).
