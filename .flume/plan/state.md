# Plan state

- Spec derived through: 0aa9e62
- Audited through: 794ca2b
- Residue swept through: 794ca2b
- This tick: INBOX DRAIN. Routed the one note (grown field schemas, observed at
  4c428b5) into GROWN-FIELD-SCHEMAS, `per` builtins.md "The shipped kinds" (the
  documented fields modulate per member as ordinary declared fields). Re-verified
  the gap on disk at abd9a58 (window 4c428b5..HEAD = only the flume inbox commit):
  Skill has description/license/disable-model-invocation/user-invocable/paths/prose;
  Agent has only description/prose — the reported grown keys genuinely absent.
  Field folding is generic (kind.ts Object.entries minus FRAMEWORK_KEYS) + generic
  frontmatter adapter → SDK-typing-only, no Rust/lock ripple. Clauses excluded and
  left to the (builtins-coverage-predicates) fork per the note. Cursors copied
  verbatim — no audit/sweep/spec-delta job this tick (abd9a58 is flume-only).
- Queue: GROWN-FIELD-SCHEMAS (open, pickable) · PACKAGING-CHANNELS-REMAINDER
  (parked — John's Apple notarizing + v0.1 tag). Disjoint files.

Plan continues: no — inbox drained, spec delta empty, no unreconciled src/ commit
past 794ca2b (abd9a58 is flume-only). GROWN-FIELD-SCHEMAS is open and pickable, so
build takes over.
