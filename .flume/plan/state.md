# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: 73c76ca — unchanged; no commit past it has landed.
- Residue swept through: 73c76ca — unchanged; no commit past it has landed.
- Posture swept through: judges next (mid-rotation) — unchanged, copied
  forward verbatim; this tick touched no judges-subsystem file
  (engine/graph/dial/coverage/coverage_note/display/reporter).
- This tick: INBOX — drained the two notes 73c76ca appended, both stamped
  `observed at 88da37f`. Note 1 (DEFECT, RULED under the 07-18 delegation):
  investigated on disk and routed as DRIFT-INCLUDE-SOURCE-PATH-CWD-LEAK
  (`per`: specs/model/pipeline.md, "Emit"). Traced the mechanism past the
  reporter's symptom to its concrete site: `harness_relative` (drift.rs
  1826-1840), the sole caller of which is the composed-prose include loop
  in `emit`'s member loop (1115). It absolutizes `harness_root` via
  `std::path::absolute` (1835) — lexical, cwd-prepending, no symlink
  resolution — while `include.source_path` arrives already absolute from
  the SDK's Node subprocess, anchored to a `fs::canonicalize`d (symlink-
  resolved) entry path (`run_sdk_program`, 845); when the two absolute
  forms disagree, `strip_prefix` (1836) silently fails and the row is
  written as a raw, host-and-cwd-dependent absolute path (1838) — never
  refused. Filed `open` (a real correctness defect, not a design fork) and
  placed queue-front, since it shares src/drift.rs with the already-open
  DRIFT-EMIT-LOCK-PARSE-HOIST — serialized that entry behind it (gate
  flipped open → blockedBy, notes trimmed to stay in budget), a
  precedence call (defect outranks a cost-hoist cleanup) rather than a
  functional dependency. Note 2 (NOTE, friction drained 07-18): recorded
  as already actioned — its own text states the live residue (the
  discriminated-union predicate) was put to John directly, "not an entry,
  not a fork yet," and the hop-cap duplicate was already dropped as
  tracked — nothing left to route; drained with no new pending entry or
  open-question. Both notes removed from `.flume/inbox.md`, which is now
  empty (its template header only).
- Queue: 30 pending — 3 pickable OPEN (DRIFT-INCLUDE-SOURCE-PATH-CWD-LEAK,
  newly filed and queue-front; BUILTIN-KIND-DEFINITIONS-RESULT-COLLAPSE;
  JSON-MANIFEST-TOP-LEVEL-OBJECT-PARSE-CONSOLIDATE — pairwise
  file-disjoint, re-checked this tick), 25 chained blockedBy (all resolve
  to live tags, re-checked this tick — DRIFT-EMIT-LOCK-PARSE-HOIST moved
  from open into this set), 2 parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER — untouched this tick). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. Refactor captures:
  0 live (unchanged). Friction: 0 live (unchanged). Inbox: **0 notes** —
  fully drained this tick. Disjointness re-checked: no two OPEN entries
  share a file; 30 unique tags, no duplicates; every blockedBy tag
  resolves.

Plan continues: yes — the posture sweep is still mid-rotation (judges
next) and no commit past 73c76ca has landed to re-trigger reconciliation,
so the sweep is the next live input.
