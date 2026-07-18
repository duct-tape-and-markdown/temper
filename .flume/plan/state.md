# Plan state

- Spec derived through: 506c34c — decision 0042 ("the entry declares
  its shape") routed this tick.
- Audited through: 64828d9 — unchanged, not this tick's job.
- Residue swept through: 64828d9 — unchanged, not this tick's job.
- Posture swept through: judges next (mid-rotation) — pipeline read
  and swept two ticks ago (ab01fb4); not this tick's job.
- This tick: SPEC DELTA. `git log 64828d9..HEAD -- specs/` names one
  commit, 506c34c (decision 0042: a collection's manifest-entry value
  decomposes per a closed, kind-declared shape — object/scalar/
  group-array — replacing today's `CollectionKeyPath`-identity
  branches in json_manifest.rs/drift.rs and retiring the
  `ENABLEMENT_FIELD` engine constant into the enabled-plugins kind's
  own declared shape). Routed every Consequences bullet by name:
  - Bullet 1 (kind.rs gains the entry-shape enum; ENABLEMENT_FIELD
    retires) — split across two entries per the retirement's two
    consumers: KIND-ENTRY-SHAPE-DATA-DECLARE (the kind.rs data model,
    blockedBy KIND-ZERO-CONSUMER-EXPORTS-PRUNE for shared kind.rs
    safety) does NOT delete the constant yet (json_manifest.rs/
    graph.rs both still read it); GRAPH-ENABLEMENT-FIELD-RETIRE
    (blockedBy JSON-MANIFEST-ENTRY-SHAPE-DISPATCH, last in the chain)
    completes it once both consumers are gone — traced on disk that
    graph.rs's dead_registration has no CollectionAddress in hand
    today (only bare Registration/Features), so this is a real,
    not cosmetic, follow-on the decision's own text doesn't name but
    the retirement cannot complete without.
  - Bullet 2 (json_manifest.rs/write-faces re-key; identity branches
    delete) — JSON-MANIFEST-ENTRY-SHAPE-DISPATCH (blockedBy
    KIND-ENTRY-SHAPE-DATA-DECLARE), folding in drift.rs's emit()
    identity branches the decision's prose bundles under "the write
    faces."
  - Bullet 3 (shipped kinds declare shapes with their cites; lock/
    schema rows round-trip) and bullet 4 (SDK mirrors, ts-rs
    regenerates) — both folded into KIND-ENTRY-SHAPE-DATA-DECLARE
    (builtin_kind.rs's four CollectionAddress literals already carry
    the needed format cites in their doc comments; CollectionAddressRow
    in drift.rs plus sdk/src/kind.ts + declarations.ts).
  - Bullet 5 (gauntlet gains a fixture composing the three shapes) —
    verified-already-moot on disk this tick: tests/gauntlet.rs's
    composition 3 (116-126) already composes hook (group-array),
    installedPlugin (scalar), and knownMarketplace (object) in one
    settings.json — filed as regression coverage inside
    JSON-MANIFEST-ENTRY-SHAPE-DISPATCH's tests[] rather than a new
    entry.
  - Bullet 6 (no fork record to delete) — verified-already-moot: no
    open-questions.md entry exists naming this question; nothing to
    delete.
  A dedicated general-purpose agent mapped every touched symbol with
  file:line citations before entries were drafted (kind.rs's
  CollectionAddress/CollectionKeyPath/ENABLEMENT_FIELD, json_manifest.rs's
  manifest_members and its five grammar functions, drift.rs's two
  identity branches inside emit() and the CollectionAddressRow lock
  row, graph.rs's dead_registration, the SDK mirror, and which existing
  tests construct CollectionAddress directly vs. observe behavior only)
  so all three entries cite exact current line numbers rather than the
  decision's own prose. Two design forks the mapping surfaced (the
  object-shape write's promotion to a named function; the
  Registration::Enablement-widen vs. thread-CollectionAddress choice
  for graph.rs) are left as named judgment calls for build, per this
  queue's established convention — neither is a genuine open question,
  both are closed, small, precedented choices.
- Queue: 44 pending (+3 this tick, chained: KIND-ENTRY-SHAPE-DATA-
  DECLARE blockedBy KIND-ZERO-CONSUMER-EXPORTS-PRUNE,
  JSON-MANIFEST-ENTRY-SHAPE-DISPATCH blockedBy the former,
  GRAPH-ENABLEMENT-FIELD-RETIRE blockedBy the latter). 9 pickable OPEN
  (unchanged from last tick — all three new entries are blockedBy),
  32 chained blockedBy, 3 parked on human action. Open forks unchanged:
  (multi-harness-projection), (lazy-grounds), neither touched — decision
  0042 resolved in session with no standing fork record. Refactor
  captures: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — inbox is drained and spec delta is now fully
routed (cursor advanced to 506c34c), but the posture-sweep rotation is
still mid-cycle: `judges` is untouched since fe3ff3f and skip-forwards
in bulk, landing on `provider` (builtin_kind.rs), already known-touched
by 404b73a, as the next live subsystem to read and sweep.
