# Plan state

- Spec derived through: cc5a9b3
- Audited through: d97a704
- Residue swept through: d97a704
- This tick: REFACTOR DRAIN — `build-projection-path-two-homes.md`, filed by
  1ca1f9b, was the tick's first live input (the inbox is empty; a live capture
  outranks the unreconciled d97a704..74f4e62 window). Claim re-verified at
  HEAD and it holds exactly as filed: `member_projection_path`
  (`src/drift.rs:579`) and `projectionPath` (`sdk/src/emit.ts:127`) derive one
  harness-relative path in two languages, branch for branch, and nothing
  compares them — `emit.ts:119`'s own doc comment states "the two must agree"
  as an invariant held by prose alone, and 1ca1f9b's SDK tests assert only the
  SDK's own side. Filed as **PROJECTION-PATH-SEAM-GATE** (`per`
  engineering.md, "One job, one home"), capture DELETED. The entry is the
  *gate*, not the unification that section's priority order would normally
  demand: the duplication is forced — `render` is erased at the seam, so the
  engine cannot supply the path — and the capture's own inversion alternative
  (the SDK reading a path column back) contradicts the one-way pipe
  (pipeline.md, "The SDK"). Both refusals ride the entry rather than waiting
  for build to rediscover them. Shape ruled *against* the capture's
  suggestion: it asked for a symbol-to-symbol comparison, needing a `pub`
  widen on a private engine fn plus an SDK-internal export; the entry gates
  the **property** instead (a rendered link resolves to the file emit wrote),
  which needs neither and covers `relativeProjection` too.
  Closing checklist: FORMAT-OMITS-EDGE-CLAUSE's `blockedBy` reason went false
  when 1ca1f9b shipped its blocker, so the gate flips `open` — its cites
  (contract.rs:81/475, engine.rs:97/114) are unmoved, the window having
  touched no Rust file. The `renderMemberFence` record is **discharged and
  deleted** (1ca1f9b carried it as a declared rider; the symbol greps nowhere
  in the tree). One record added: `drift.rs:570` calls the SDK
  `projectionPath` "retired" when 1ca1f9b revived it.
- Queue: 2 pickable and disjoint by file (FORMAT-OMITS-EDGE-CLAUSE, unblocked
  this tick; PROJECTION-PATH-SEAM-GATE, new — one test file);
  PACKAGING-CHANNELS-REMAINDER parked (John's Apple notarizing + the v0.1
  lockstep tag). Fork board unmoved: `(nested-file-child)` still blocks the
  supporting-doc adoption; `(guidance-climb)` blocks nothing.

Plan continues: yes — post-ship reconciliation of the d97a704..74f4e62 window
(1ca1f9b, EMBEDDED-FORMAT-TARGET-FACTS). This tick verified only what the
capture's claim touched: `emit.ts`'s path derivation and the entry gates
behind it. The audit's own motions are unrun — the four target facts against
`representation.md`'s closed set, the two shipped refusals, and the sweep of
`emit.ts`/`kind.ts` against the corpus — so both cursors stay at d97a704.
