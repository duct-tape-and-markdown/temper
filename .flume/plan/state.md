# Plan state

- Spec derived through: f7d870c
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: foundation done — model next
- This tick: SPEC DELTA. f7d870c (0040 — the tension edges resolve toward
  the map) was the one commit past the prior cursor (663e03f); f404e48
  (the companion `chore(flume)` commit) had already deleted the three fork
  records `(drift-install-edge)`, `(frontmatter-builtin-kind-edge)`,
  `(extract-foundation-edge)` from open-questions.md, explicitly deferring
  entry derivation to this tick ("plan derives the entries from the spec
  delta"). Routed every bullet of 0040's Consequences section, the
  derivation checklist:
  - "`src/placement.rs` — new flat module... `install` and `drift` import
    it" → filed `PLACEMENT-MODULE-EXTRACTION`.
  - "architecture.md: the tension paragraph is replaced..." → already
    shipped, in f7d870c itself; verified on disk (the corpus excerpt
    above already reads the ruled directions).
  - "`test_support` gains a synthetic kind-builder; `frontmatter`'s test
    module drops its `builtin_kind` import" → filed
    `FRONTMATTER-TEST-SYNTHETIC-KINDS`.
  - "`extract.rs` ends with no crate-internal imports; `drift.rs:2603`'s
    doc-comment pointer... rewords in scope of the placement entry" →
    the extract.rs half filed as `EXTRACT-FOUNDATION-BOUNDARY-RESTORE`;
    the drift.rs:2603 doc-pointer half folded into
    `PLACEMENT-MODULE-EXTRACTION`'s drift.rs edit (verified on disk:
    `install::matches_projection` is prose-only there, unrelated to the
    moved vocabulary — instructed to reword only if it reads stale, said
    in the entry, not invented as a forced change).
  - "Pending `EXTRACT-PRIVATE-COLLECTION-DECODERS` reconciles against the
    moves..." → folded into `EXTRACT-FOUNDATION-BOUNDARY-RESTORE`'s
    acceptance (manifest_members/entry_fields/hook_member_fields/
    enablement_member_fields private at their new json_manifest.rs home,
    enablement_entry_value/hook_matcher_group staying pub(crate) for
    drift); the superseded `EXTRACT-PRIVATE-COLLECTION-DECODERS` entry
    removed from pending.json.
  - "The three fork records delete" → already done, f404e48.
  All three code-move entries verified against current HEAD (f404e48) by
  direct grep/read (line numbers, call sites, doc-comment pointers) before
  filing — not re-derived from the decision's prose alone. Every path in
  `pending-entry.md`'s "Disjoint, or serialized" bar re-checked: the new
  entries and two already-open entries (DISCOVERY-INFALLIBLE-RESULT-
  COLLAPSE, DRIFT-LOCK-ROW-WALK-CONSOLIDATION) collide pairwise on
  src/drift.rs, src/install.rs, src/json_manifest.rs, and src/main.rs, so
  all five chain into one strict `blockedBy` sequence (DISCOVERY →
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION [gate changed, no direct overlap but
  needed for the transitive chain] → DRIFT-EMIT-LOCK-PARSE-HOIST →
  PLACEMENT-MODULE-EXTRACTION → EXTRACT-FOUNDATION-BOUNDARY-RESTORE) —
  the schema's single-tag `blockedBy` cannot express "wait for two
  independent prior entries" otherwise. `FRONTMATTER-TEST-SYNTHETIC-KINDS`
  stays open, disjoint from every other entry's files.
- Queue: 8 pending — 2 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-COLLAPSE,
  FRONTMATTER-TEST-SYNTHETIC-KINDS; disjoint files), 4 chained blockedBy
  (DRIFT-LOCK-ROW-WALK-CONSOLIDATION → DRIFT-EMIT-LOCK-PARSE-HOIST →
  PLACEMENT-MODULE-EXTRACTION → EXTRACT-FOUNDATION-BOUNDARY-RESTORE), 2
  parked on human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-
  REMAINDER). Open forks: (multi-harness-projection), (lazy-grounds)
  unchanged — the three architecture forks are gone (resolved, not
  parked). No live refactor captures; inbox empty.

Plan continues: yes — posture sweep resumes at `model`, the roster's next
subsystem (mid-rotation per `Posture swept through:`), once nothing above
it is live.
