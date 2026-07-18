# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: d40a9f8 — unchanged; `git log d40a9f8..HEAD -- src/
  tests/ sdk/` is empty (all six commits since — 5af93d9, 3871eba,
  9e197d6, 7a5f86c, 69e7571, 662cf07 — are plan-only).
- Residue swept through: d40a9f8 — unchanged, same empty window.
- Posture swept through: 662cf07 — unchanged, mid-rotation. This tick
  swept `judges`; foundation/model/formats/pipeline/judges stay
  ticked, next: provider.
- This tick: POSTURE SWEEP — judges subsystem (engine.rs, graph.rs,
  dial.rs, coverage.rs, coverage_note.rs, display.rs, reporter.rs)
  swept against every section of `specs/process/engineering.md`,
  cross-checked against the queue to avoid re-flagging in-flight work
  (graph.rs's four open chains, coverage_note.rs's lock-parse-hoist
  chain, dial.rs's severity-label consolidation). Three new findings
  filed, each serialized behind the last existing chain entry touching
  its shared file rather than left `open`:
  - **COVERAGE-NOTE-GOVERNS-FILE-LEAF-CONSOLIDATE** ("One job, one
    home") — `governs` (345-365) derives the identical
    root/glob-leaf file match twice in its own body and
    `governs_segment` (372-382) repeats it a third time, verified
    byte-identical on disk. Serialized behind COVERAGE-NOTE-LOCK-
    PARSE-HOIST, the current last chain entry touching coverage_note.rs.
  - **GATE-MANIFEST-SHARED-READ-HOIST** ("Cost scale is hoisted, and
    pinned by count") — three built-in kinds (`hook`,
    `installed-plugin`, `known-marketplace`) all govern
    `.claude/settings.json`; each's `manifest_units` call independently
    opens+parses it via `read_kind`, and `coverage_note`'s
    `manifest_top_level_keys` reads it again for coverage — up to four
    independent parses of one file per gate()/explain() run, verified
    on disk (`Manifest::read`'s own `addresses: &[&CollectionAddress]`
    signature already supports the combined read no caller uses).
    Serialized behind JSON-MANIFEST-DISCOVERY-BOUNDARY-RESTORE, whose
    `read_kind` narrowing this builds on; also shares coverage_note.rs
    with the sibling entry above (single-tag `blockedBy` can't express
    both waits, noted in the entry).
  - **ENGINE-PREDICATE-FENCE-EXHAUSTIVE-MATCH** ("A shared concept is
    one type") — `bodyless`, `judgeless`, and `vacuities` (engine.rs)
    each close their `Predicate` match with a `_ =>` wildcard, unlike
    this file's own `addressed_field`/`decide`/`judge`, all already
    exhaustive over the same 27-variant enum — the same shape as the
    already-queued KIND-DECLARED-FIELDS-EXHAUSTIVE-MATCH/CONTRACT-
    DECLARED-KEYS-EXHAUSTIVE-MATCH precedent. Serialized behind
    GRAPH-ENGINE-GLOB-EXTRACTOR-CONSOLIDATE, last chain entry touching
    engine.rs.
  dial.rs, display.rs, and reporter.rs swept clean (every pub/pub(crate)
  export has a verified outside-module consumer, every shared-enum
  match is exhaustive, no vacuous Result paths). dial.rs's dead private
  `is_empty` (already `#[allow(dead_code)]`, not pub) is sub-threshold
  — not filed. graph.rs surfaced no new finding beyond its four already-
  open chain entries.
- Queue: 34 pending (31 + 3 filed this tick). 6 pickable OPEN
  (unchanged — all three new entries are blockedBy), 26 chained
  blockedBy, 2 parked on human action. Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. Refactor
  captures: 0 live. Inbox empty.

Plan continues: yes — posture sweep resumes at `provider` (builtin.rs,
builtin_kind.rs), the rotation's next subsystem.
