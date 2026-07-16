# Plan state

- Spec derived through: fd52717
- Audited through: 74f4e62
- Residue swept through: 74f4e62
- This tick: ROUTED the spec delta's first commit — fd52717 (0026, an unfilled
  edge field is no edge). The delta carries two decisions; 0027 (abe5d5d) is
  the larger slice and waits its own tick, so the cursor stops at fd52717.
  **0026's Consequences, bullet by bullet:** (1) "`pipeline.md`'s 'Refusing'
  bullet gains the boundary sentence — same commit" — moot, verified in
  fd52717's own diff (pipeline.md, "Emit", the four-line insert). (2) "the
  emit throw retreats to dangling-only" + (3) "the schema's optionality
  reaches the declared fact row so the engine sees what the author declared"
  — both filed as **UNFILLED-EDGE-FIELD-NO-EDGE**, one entry because they land
  together or ship a false positive: emit's throw (`sdk/src/emit.ts:186-193`)
  is what keeps an unfilled edge field from ever reaching the engine today,
  and `embedded_member_features` (`src/main.rs:1481`) crosses the kind's
  *whole* declared edge set with the row's `placed_edges` — so the moment the
  throw retreats, a legitimately unfilled optional field reads `false` and
  `format-places-edges` (13c58ed) indicts a format for an edge the value never
  carried. Invariant 2's false positive.
  **The reading bullet 3 rests on, stated for the veto:** optionality reaches
  the engine through the row emit already writes — `NestedMemberRow.leaves`
  names the filled fields — never a new column. The rival reading (an
  optionality marker on the `assembly` `edge` fact row) is unbuildable by
  0026's own Rejected list: it forbids a flag on `EdgeField`, and the field
  schema's one runtime spelling is the type parameter `T`, erased at the seam
  (`sdk/src/kind.ts:250`). Required-vs-optional need never reach the engine —
  0026 puts the unfilled *required* field's failure in the author's program at
  compose time, so absent is no edge either way.
  Closing checklist: the two entries are disjoint by file (`sdk/**` +
  `src/main.rs` + `tests/contract_template.rs` vs
  `.github/workflows/release.yml`). PACKAGING-CHANNELS-REMAINDER's park
  re-tested at ca4e866 and true verbatim: 4 tags, all era-named (no version
  tag), crate 0.1.0 vs npm 0.0.7, release.yml:7-9 still defers darwin. Every
  cited line resolves on disk (emit.ts:186/344, kind.ts:43/356,
  refusals.test.ts:260, main.rs:1481). The fork board holds at four records —
  `(edge-field-floor)`, `(nested-file-child)`, and `(guidance-climb)` were
  resolved and deleted by 8769ed8, so it needs no edit this tick; the prior
  state's "grows by one" line is retired by that ruling.
- Queue: 1 pickable (UNFILLED-EDGE-FIELD-NO-EDGE);
  PACKAGING-CHANNELS-REMAINDER parked (John's Apple notarizing + the v0.1
  lockstep tag). Fork board: four open, none blocking either entry.

Plan continues: yes — the spec delta is still live. abe5d5d (0027, the nested
file child composes its path from its host) is unrouted, and its Consequences
name three code-side halves plus SKILL-NESTED-REFERENCE-DOCS re-entering
buildable. The post-ship reconciliation window (74f4e62..ca4e866 — two build
commits, 13c58ed and e76ec85) is live behind it.
