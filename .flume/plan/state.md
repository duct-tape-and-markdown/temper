# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: d40a9f8 — unchanged; `git log d40a9f8..HEAD -- src/
  tests/ sdk/` is empty (all five commits since — 5af93d9, 3871eba,
  9e197d6, 7a5f86c, 69e7571 — are plan-only).
- Residue swept through: d40a9f8 — unchanged, same empty window.
- Posture swept through: 5af93d9 — unchanged, mid-rotation. This tick
  swept `pipeline`; foundation/model/formats/pipeline stay ticked,
  next: judges.
- This tick: POSTURE SWEEP — pipeline subsystem (drift.rs, import.rs,
  read.rs, builtin_lock.rs; `placement.rs` not yet shipped, skipped)
  swept against every section of `specs/process/engineering.md`, cross-
  checked against the queue to avoid re-flagging in-flight work. Two
  new findings filed, both "One job, one home", both serialized behind
  the last existing chain entry touching their shared file rather than
  left `open` (drift.rs and graph.rs both already carry open chains):
  - **DRIFT-WRITE-PLACEMENT-CONSOLIDATE** — `write_placement`
    (drift.rs:2298) is the named mkdir+write+`DriftError::Write`
    helper, but `emit_manifest` (1475-1486) and `emit_one`
    (2071-2082) each reimplement its body inline instead of calling
    it — three copies of one job in one file, verified on disk (all
    three call sites read byte-identical). Serialized behind
    NORMALIZE-PATH-SUBSYSTEM-PLACEMENT, last chain entry touching
    drift.rs by array position; DRIFT-EMIT-OUTCOME-LABEL-ZERO-
    CONSUMER-PRUNE also touches drift.rs on a parallel branch —
    single-tag `blockedBy` can't express both waits, noted in the
    entry.
  - **READ-GRAPH-DIRECTIVE-FIELD-CONSOLIDATE** — read.rs's
    `DIRECTIVE_FIELD_LABEL` (1226) is a hand-maintained duplicate of
    graph.rs's private `DIRECTIVE_FIELD` (43), self-documented in its
    own doc comment as "the mirror of `graph`'s private
    `DIRECTIVE_FIELD`" with no compiler-checked link between the two.
    Serialized behind GRAPH-ENGINE-GLOB-EXTRACTOR-CONSOLIDATE, last
    chain entry touching graph.rs — its blockedBy chain already
    passes through GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE, read.rs's
    only other pending toucher, so one blocker covers both files.
  import.rs and builtin_lock.rs swept clean (glob compilation and
  discovery walks are properly hoisted with count-pins; no unwarranted
  duplication, no zero-consumer pub surface, no vacuous Result paths).
  Several other leads (ImportError/NestedFileUnit zero-consumer,
  read.rs's Citation, graph.rs's two `_ =>` arms) were investigated and
  ruled out as false positives or already-documented placeholders —
  not refiled.
- Queue: 31 pending (29 + 2 filed this tick). 6 pickable OPEN
  (unchanged — both new entries are blockedBy), 23 chained blockedBy,
  2 parked on human action. Open forks: (multi-harness-projection),
  (lazy-grounds) unchanged. Refactor captures: 0 live. Inbox empty.

Plan continues: yes — posture sweep resumes at `judges` (engine/graph/
dial/coverage/coverage_note/display/reporter), the rotation's next
subsystem.
