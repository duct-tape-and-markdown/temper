# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: d40a9f8
- Residue swept through: d40a9f8
- Posture swept through: 1a2b753 — rotation closed last pass; re-arms
  now, since `git log 1a2b753..HEAD -- src/ sdk/src/ tests/` is
  non-empty (this tick's own reconciled window touched src/) — next
  tick opens a fresh cycle at foundation.
- This tick: POST-SHIP RECONCILIATION — window f713a08..d40a9f8.
  Shipped: DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH (87221b2),
  ENGINE-JUDGE-SELECTION-EXHAUSTIVE-MATCH (d02605a), DIAL-IS-EMPTY-
  ZERO-CONSUMER-PRUNE (059c3fd), reconciled out of pending.json by
  d40a9f8. Audit: re-verified each ship on disk (never the log alone)
  and re-tested the three downstream blockedBy gates naming them —
  JSON-MANIFEST-READ-DECODE-CONSOLIDATE and TOML-DOCUMENT-PARSE-ZERO-
  CONSUMER-PRUNE (blocker DOCUMENT-IDENTITY-...; json_manifest.rs's
  DocumentMember::read/Manifest::read unmoved, toml_document.rs's
  parse widened +6 lines, 53-99 → 53-105) and COMPOSE-DIAL-SEVERITY-
  LABEL-CONSOLIDATE (blocker DIAL-IS-EMPTY-...; dial.rs's
  severity_from_label shifted +1 line, 144-150 → 145-151) — all three
  re-verified and flipped to open. Glanced `.flume/metrics.jsonl` per
  pending-entry.md's smart-zone bullet: a build attempt at BUILTIN-
  KIND-DEFINITION-RESULT-COLLAPSE (181 turns, ~765s, ~19.5M cache-read
  tokens) never shipped — working tree clean, disk unchanged on
  re-check — after four earlier plan-phase re-touches of the same tag.
  Split it into a blockedBy chain by function: BUILTIN-KIND-DEFINITION-
  RESULT-COLLAPSE (definition() only, ~17 files) stays open; BUILTIN-
  KIND-DEFINITIONS-RESULT-COLLAPSE (definitions() plus the KindError
  deletion, blockedBy the former, ~8 files) chains behind it — the two
  overlapping files (main.rs, tests/nested_member.rs) now land
  serialized instead of racing. KIND-DECLARED-FIELDS-EXHAUSTIVE-MATCH's
  blockedBy retargeted to the new Part B, the one that actually touches
  kind.rs. Sweep: no residue found in the shipped window beyond the
  split above; DIAL-IS-EMPTY-...'s added `#[allow(dead_code)]` matches
  established precedent (address.rs, test_support.rs already carry it,
  and "a test counts" as an earned consumer per engineering.md) — not
  residue.
- Queue: 27 pending (+1, the split), 6 pickable OPEN (BUILTIN-KIND-
  DEFINITION-RESULT-COLLAPSE, JSON-MANIFEST-READ-DECODE-CONSOLIDATE,
  TOML-DOCUMENT-PARSE-ZERO-CONSUMER-PRUNE, COMPOSE-DIAL-SEVERITY-LABEL-
  CONSOLIDATE, CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE, DRIFT-
  LOCK-ROW-WALK-CONSOLIDATION — pairwise file-disjoint, verified), 19
  chained blockedBy (was 21; three unblocked to open, one new chained
  link added by the split), 2 parked on human action (IMPORT-HOP-CAP-
  CITE, PACKAGING-CHANNELS-REMAINDER — reasons unchanged, untouched
  this window).
  Open forks: (multi-harness-projection), (lazy-grounds) unchanged.
  Refactor captures: none live. Inbox empty.

Plan continues: yes — posture sweep re-arms: `git log 1a2b753..HEAD --
src/ sdk/src/ tests/` is non-empty (this window's own ships touched
src/), so the next tick opens a fresh rotation cycle at foundation.
