# Plan state

- Spec derived through: 725188e
- Audited through: b85df4a
- Residue swept through: b85df4a
- This tick: DERIVE spec delta 725188e (decision 0035 AMENDED, "extent
  subsumes max_lines") — re-derived after last attempt (915df82) was reverted
  at the `entry references resolve` gate. Root cause: it put symbol addresses
  (`Predicate::MaxLines`, `maxLines constructor`) in EXTENT's `retire` array,
  but `retire` means "this FILE is deleted" (`pending-entry` rule) — both
  files survive, so the fence read the strings as missing file paths. Fixed:
  the max_lines retirement is symbol-within-surviving-file, so it is `edit`s;
  `retire` stays `[]`.
  Amendment Consequences (its amended section, each routed) → all to
  **EXTENT-PREDICATE** (per contract.md, "clause"):
  - "extent subsumes max_lines, which retires" → EXTENT folds the retirement
    in (remove `Predicate::MaxLines` variant/parse/label/field/eval; `extent`
    takes the node-scope slot).
  - "The shipped defaults re-spell (maxLines(500)/maxLines(200) → extent over
    the same selections), superseding 0035's 'no shipped default adopts
    extent'; no NEW budget opinion" → EXTENT edits `sdk/src/builtins.ts` (3
    adoptions) and `src/builtin_lock.toml` re-derives. `command` reuses
    `skillDefaultContract` (builtins.ts:1191), so 3 source edits flip 4 lock
    rows.
  - "Retirement is loud — a lock still carrying max_lines refuses at load" →
    pinned in `tests/extent.rs`.
  - "Plan derives the entry" (singular) → one atomic entry.
  Two corrections vs. the reverted attempt: (1) ADMISSION-JOINS-FILE-TEMPLATE
  shipped (631bc83) since it, so EXTENT's `blockedBy` is stale — gate now
  **open**, disjoint from the two parked entries. (2) Wider ripple caught: it
  missed `manifest_schema_oracle.rs`, `closed_keys.rs`,
  `sdk/test/{emit,builtins}.test.ts`, `sdk/src/index.ts` — all now in
  `files.edit`. The `(extent-subsumes-max-lines)` fork was already deleted by
  the human in 725188e (encode + delete) — re-verified gone. Build-confirm
  flagged, not invented: `body_lines`/`LineCount` (kind.rs) may orphan once
  max_lines' eval goes; 0035 names no retirement for it.
- Queue: 3 entries — EXTENT-PREDICATE **pickable** (gate:open) + 2 parked
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Disjoint. No fork rest.

Plan continues: yes — spec delta still live: 63e1f22 (0036, settings-local
kind) then 6d2cca6 (0037, typed verifier) un-derived, one slice per tick;
then post-ship reconcile of b85df4a..HEAD (3 build commits).
