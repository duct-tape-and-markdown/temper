# Plan state

- Spec derived through: 832f015
- Audited through: 0c3cbcb
- Residue swept through: 0c3cbcb
- This tick: DRAIN the refactor capture SHAPE-PREDICATE filed
  (`build-schema-docs-channel-single-slot.md`, observed 94012c4) — the
  directory held a live capture, so the inbox job outranked the window's
  reconciliation. Filed SCHEMA-DOCS-CHANNEL-ACCUMULATES; capture deleted.
  **The claim verified at HEAD, and the sharper number is worse than the
  report's.** Not off the log: `src/schema.rs`'s docs-channel loop (169-186)
  `insert`s `description` per clause into the shared property map, and
  `insert` is last-writer-wins, so a field's 2nd..Nth guided clause overwrites
  its predecessor with nothing reporting the loss. Confirmed empirically by
  running the shipped binary rather than reading the code alone: `temper
  schema --kind skill` shows `description` carrying only the `shape`
  teaching — the spec's 1024-cap prose is gone from hover — and `name`
  carrying only `deny`'s, five teachings behind it dropped.
  **Scope re-derived from the clause data, not the report.** The capture named
  one field; counting the built-in lock's guided clauses over the eleven
  predicates `documented_field` answers gives **8 live dropped teachings**
  (`skill.name` 5, `skill.description` 3 — the `schema` verb models `skill`
  and `rule` alone) and **14 latent** (`command.name` 5, `command.description`
  3, `marketplace.name` 3, `plugin-manifest.name` 2, `agent.name` 1) that land
  the moment those kinds get a schema face. **One report claim did not
  survive**: `rule.paths` looks like a two-clause field but is not affected —
  `mention-reachable` returns `None` from `documented_field`
  (`src/contract.rs`), so it never writes. Scoped to the verified gap, not the
  reported one.
  The entry cites `engineering.md`, "One job, one home" on the report's own
  logic: `push_subschema` (`schema.rs`:318) is the accumulation home
  SHAPE-PREDICATE built for this exact hazard on `pattern` one channel over,
  and the docs loop is that hazard's twin. Join over pick-a-winner is the
  corpus's call, not the queue's — `builtins.md` binds guidance to ride "the
  clause value itself, so it cannot dangle from the check it explains", and
  the pick-a-winner alternative needs a rule for which clause wins that
  nothing states; author order decides by accident today. No fork registered:
  the corpus already rules this.
  **Closing checklist moved one gate.** DIAL-KIND rested on SHAPE-PREDICATE,
  which shipped inside this window (0927979) and left the queue with the ship
  commit — a gate pointing at a tag no longer in it. Verified on disk before
  opening, not from the log: `Predicate::Shape` (`contract.rs`:241), its
  decoder arm (473), the `schema.rs` arm (121), and `tests/shape_predicate.rs`
  all present. It is now `open` — the fifth wave head in a row to open this
  way — so the queue carries **2 pickable, disjoint** entries. DIAL-KIND's two
  SDK cites (`sdk/src/builtins.ts` +24, `sdk/src/index.ts` +1: `shape` at 50)
  went stale in the same window; the entry says so in place rather than
  reading as freshly scoped, and they re-stamp at next tick's audit — which is
  where that motion belongs, not smuggled into a drain tick.
  Cursors copied forward verbatim: the audit and sweep of `0c3cbcb..a2e48aa`
  are untouched, and a capture drain is not an audit.
- Queue: 5 entries, **2 pickable** — SCHEMA-DOCS-CHANNEL-ACCUMULATES (ordered
  first: a live defect in a shipped channel, one file, disjoint from the wave)
  and DIAL-KIND. CHECK-ANNOUNCES chains behind the dial on `src/main.rs`; no
  entry rests on a fork. Two parked.

Plan continues: yes — post-ship reconciliation of `0c3cbcb..a2e48aa` (both
cursors sit at 0c3cbcb; SHAPE-PREDICATE's ship is unaudited and unswept). The
window is one build commit touching `src/`, `tests/`, and `sdk/`, and it
carries known audit work already named above: DIAL-KIND's two stale SDK cites,
the `(source-union-predicate)` record's now-true antecedent — the skill's two
holds retired (`builtins.ts`:1058 reads "Nothing decidable is held"), leaving
the marketplace's `source` union the last hold in the provider face, the
prediction's third payout — and the `builtins.ts` re-cite the marketplace
header rewrite implies.
