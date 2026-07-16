# Plan state

- Spec derived through: abe5d5d
- Audited through: ca4e866
- Residue swept through: ca4e866
- This tick: RECONCILED the 74f4e62..ca4e866 window — two build commits,
  13c58ed (FORMAT-OMITS-EDGE-CLAUSE) and e76ec85 (PROJECTION-PATH-SEAM-GATE),
  both already dropped from pending by ca4e866.
  **Audit — verified on disk, not from the log:** the clause shipped whole —
  `Predicate::FormatPlacesEdges` (`src/contract.rs:261`, spelled
  `format-places-edges` at 354/428), its judge (`src/engine.rs:464-482`, empty
  map ⇒ `Indeterminate`), the `placed_edges` lock column
  (`src/drift.rs:2596-2613`, `Option<Vec<String>>`, `None` for a layout-hosted
  value at 1288-1292), `Features::edge_placements` (`src/extract.rs:329-338`),
  the SDK's recording view (`sdk/src/emit.ts` `placedEdges`, 340-350), and
  `formatPlacesEdges()` (`sdk/src/contract.ts:124`). No built-in adopts it —
  `src/builtin_lock.toml` unmoved. The seam gate is `tests/projection_path_seam.rs`,
  212 lines, comparing the two derivations through the property, not the symbol.
  No pending entry's work shipped; nothing to drop. **Gates re-tested, each still
  true:** UNFILLED-EDGE-FIELD-NO-EDGE open, its every cite resolving
  (`edgeTargetFacts`'s `names no target` throw at emit.ts:188-193; `placedEdges`'s
  no-`render` whole-set claim at 346; `embedded_member_features`'s
  declared-set-crossed-with-`placed_edges` map at main.rs:1508-1516 — the
  false positive the entry is scoped to). The three-entry chain behind it
  (NESTED-FILE-LOCUS → NESTED-FILE-DISCOVERY → SKILL-NESTED-REFERENCE-DOCS)
  holds on its shared paths. PACKAGING-CHANNELS-REMAINDER's park re-tested and
  true verbatim: 4 tags, all era-named, crate 0.1.0 vs npm 0.0.7,
  release.yml:7-9 still defers darwin on John's Apple notarizing.
  **Sweep — one gap filed, SDK-FIXTURE-WIRING-ONE-HOME:** `tests/common/mod.rs`'s
  own header declares it the one home for "the SDK vendoring used by tests that
  drive a real `node` subprocess … every suite was carrying its own copy of", and
  it carries `tmpdir`/`scaffold`/`vendor_sdk` — every ingredient except the
  builder that composes them into a fixture harness. That builder now exists
  three times: `wire_sdk_harness_program` (tests/emit.rs:1220), and two verbatim
  copies whose doc comments cite it by name as the pattern they follow —
  `wire_memberless_harness` (tests/builtin_lock_frozen.rs:61) and, added by this
  very window, `wire_waypoint_harness` (tests/projection_path_seam.rs:117).
  `specs/process/engineering.md`, "One job, one home": test scaffolding is a
  surface, and a builder lives in one home, never copy-pasted per file. Serialized
  behind SKILL-NESTED-REFERENCE-DOCS — it shares projection_path_seam.rs with
  NESTED-FILE-LOCUS and builtin_lock_frozen.rs with SKILL-NESTED-REFERENCE-DOCS,
  and two `open` entries over one file revert the wave.
  Closing checklist: the queue is disjoint — one chain of five
  (UNFILLED-EDGE-FIELD-NO-EDGE → NESTED-FILE-LOCUS → NESTED-FILE-DISCOVERY →
  SKILL-NESTED-REFERENCE-DOCS → SDK-FIXTURE-WIRING-ONE-HOME) plus the park.
  Fork board: four open, none blocking a queued entry — 25e9317 landed mid-tick
  and resolved `(supporting-doc-reach-clause)`, filing its ruling to the inbox;
  that is the next tick's job, not this one's. Every rider record
  re-verified on disk at ca4e866; three restamped — contract.rs's layering
  straggler (moved 475 → 490; 13c58ed opened the file and left it, the second
  entry to do so), drift.rs:570's "retired projectionPath" (unmoved; 13c58ed
  opened drift.rs and left it — its carrier stays NESTED-FILE-LOCUS, which names
  it in `files[].description`), and extract.rs's floor-mention comment (unmoved;
  13c58ed is its fifth opener). The rest (session_start.rs fixtures,
  builtins.ts:565/611, read.rs's five strand comments, prose.ts's narration,
  Cargo.toml:42-43) unmoved and untouched this window.
- Queue: 1 pickable (UNFILLED-EDGE-FIELD-NO-EDGE); four serialized behind it as
  one chain (NESTED-FILE-LOCUS → NESTED-FILE-DISCOVERY →
  SKILL-NESTED-REFERENCE-DOCS → SDK-FIXTURE-WIRING-ONE-HOME);
  PACKAGING-CHANNELS-REMAINDER parked (John's Apple notarizing + the v0.1
  lockstep tag).

Plan continues: yes — the inbox is live. 25e9317 landed while this tick's
reconciliation was in flight (it was not HEAD when the tick was dispatched) and
filed John's `(supporting-doc-reach-clause)` ruling: the reachability check is a
clause, never a requirement, plus two entries to route and a SKILL.md 500-line
budget clause candidate. Inbox outranks reconciliation, so it is the next tick's
job; this tick's window (74f4e62..ca4e866) is closed either way.
