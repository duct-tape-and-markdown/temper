# Plan state

- Spec derived through: 5945405
- Audited through: b2afc32
- Residue swept through: b2afc32
- This tick: drained the inbox (2 lines, both from e3c28e0's friction drain).
  Routed both into pending as entries — facts verified on disk first.
  (1) GENRE-FOLD residual → LOCK-NESTING-TEMPLATES (open): `KindFactRow`
  (src/drift.rs) has no templates column, `from_kind_fact_row` (src/kind.rs)
  leaves templates empty with a gap-naming comment, SDK `kindFactRow`
  filters genres out — add a templates string-array column + SDK emit +
  engine restore so the lock round-trips a kind's nesting templates. Scope =
  embedded (genre) nesting; fold keys only on child-kind name. (2) One-time
  cite retag → CITE-RETAG (blockedBy LOCK-NESTING-TEMPLATES): 60 source files
  carry retired-layout cites (architecture/NN-*, bare n//l/, l.md, 00-intent,
  "law N"); John authorized ONE comment-only sweep, serialized behind the open
  src-touching entry (shared: drift/kind/declarations). Excluded the .snap
  (fixture payload prose, not a cite).
- Audit/residue cursors trail HEAD by 3 commits (0091144 plan, e3c28e0 flume,
  4fabbde docs) — verified none touch src/tests/sdk, so no audit/sweep work is
  owed; cursors copied forward verbatim, reconciled on build's next code commit.
- Queue: 3 — LOCK-NESTING-TEMPLATES (open), CITE-RETAG (blockedBy
  LOCK-NESTING-TEMPLATES), PACKAGING-CHANNELS (parked: release creds + workflow).

Plan continues: no — inbox drained; spec delta empty (cursor 5945405); ship
audit + residue not live (3 intervening commits are non-code). LOCK-NESTING-
TEMPLATES is open and pickable, so build takes over.
