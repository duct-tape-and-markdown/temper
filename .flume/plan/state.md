# Plan state

- Spec derived through: abe5d5d
- Audited through: 4a33911
- Residue swept through: 4a33911
- This tick: ROUTED the inbox's one note — the 0024 read-robustly gap — into
  LOCK-LEGACY-TEMPLATES-READ (`per` pipeline.md, "The lock"), open. The report's
  claimed gap was re-verified against the tree before it scoped anything, and
  it survived on every clause. The refusal is real and exact: `templates_from_table`
  (`src/drift.rs:3139`) raises `RowError::wrong("templates", "array of tables")`
  at 3150 for any non-inline-table element, refusing the whole lock load before
  `emit` can rewrite a byte. The **historical premise is proven, not assumed** —
  f19f49b's diff of `examples/base-harness/.temper/lock.toml` rewrote four
  bare-string rows (`["alternative"]`, `["step"]`, `["invariant"]`) into inline
  tables, so an older SDK demonstrably emitted the spelling today's reader
  refuses. The normalizer's precedent is live SDK-side at `declarations.ts:189`
  (`admitted.map((kind) => ({ kind }))`).
  **The entry draws a line the note did not, because the naive fix is wrong.**
  `LockRowError`'s own doc (drift.rs:333-337) rejects "a row the SDK could not
  have emitted" as a corrupt lock — refuse loud, never drop. That principle and
  0024's robust read collide only if the discriminator is read as *today's* SDK;
  read as *any* SDK version, they partition cleanly: a legacy bare string is a
  spelling an older engine really wrote (0024 normalizes it), while an integer or
  array element is a row no version could emit (invariant 6 still refuses it).
  The entry names that line so build cannot discharge the finding by weakening
  the corrupt-lock refusal wholesale — which would narrow the gate against
  genuinely corrupt locks.
  Scope verified rather than inherited: no other lock column ever carried a
  string form (the sibling array-of-tables lifts at 3220/3355/3713 never had
  one), so `templates_from_table` is the entire blast radius — one lift, one
  test home. The note's `withinHosts` half is context explaining why the cliff
  has no exit, not a second job: it is the author's own source migration.
  Closing checklist: one open entry, so the queue is trivially disjoint —
  LOCK-LEGACY-TEMPLATES-READ's two files are shared with nothing pending
  (IMPORT-HOP-CAP-CITE holds graph.rs and is parked besides). Both parks
  re-tested on disk at this HEAD and hold verbatim: no version tag (only the
  four era tags), crate 0.1.0 vs npm 0.0.7, release.yml:7-9 still deferring
  darwin + channel 3; the hop constant still reads 5 and nothing in the window
  ruled its semantics. `src/graph.rs` and `tests/graph.rs` are untouched since
  4a33911, so IMPORT-HOP-CAP-CITE's line cites all still resolve — no rewrite
  earned. open-questions.md is unchanged: no fork resolved or opened, and the
  standing riders are the audit motion's to restamp, not this tick's to claim.
  Field lengths validated. All three cursors copied forward verbatim — this tick
  derived no spec and reconciled no window.
- Queue: 1 pickable (LOCK-LEGACY-TEMPLATES-READ, filed this tick); 2 parked on
  human acts (IMPORT-HOP-CAP-CITE: a hop-depth probe. PACKAGING-CHANNELS-REMAINDER:
  Apple notarizing + the v0.1 tag). No gate is stale — every one was tested this tick.

Plan continues: yes — post-ship reconciliation is live below this job. f60e1ff
(the fixture-harness wiring collapsed onto one home) and 7947235 sit past both
`Audited through:` and `Residue swept through:` at 4a33911 and touched
`tests/`, so the window f369531..HEAD owes an audit-plus-sweep tick.
SDK-FIXTURE-WIRING-ONE-HOME already shipped at 7947235 and was already absent
from pending, so that audit is a verification, not a drop.
