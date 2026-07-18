# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: 73c76ca — unchanged; no commit past it has landed.
- Residue swept through: 73c76ca — unchanged; no commit past it has landed.
- Posture swept through: judges next (mid-rotation) — unchanged, copied
  forward verbatim; this tick touched no judges-subsystem file
  (engine/graph/dial/coverage/coverage_note/display/reporter).
- This tick: SPEC DELTA — derived decision 0041 (contract.md gains the
  `when` guard-clause paragraph; the new decision record's own
  Consequences section is the checklist). Investigated the current code
  shape (Predicate enum, ClauseRow, engine.rs's decide/judge/admissibility
  matches, the SDK's contract.ts/declarations.ts, and every shipped
  default-contract "hold" in sdk/src/builtins.ts) before scoping, per the
  ratified-Decision derivation bar. Consequences checklist, enumerated:
  (1) "contract.md gains the guard paragraph" — verified already true on
  disk (the delta itself); no entry needed. (2) "Engine: when evaluation…
  gauntlet cell… vacuity-honest tests" — filed CONTRACT-WHEN-GUARD-CLAUSE-FORM
  (open, blockedBy ENGINE-PREDICATE-FENCE-EXHAUSTIVE-MATCH for shared
  contract.rs/drift.rs/engine.rs safety — that chain also carries
  CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH/CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP,
  so `declared_keys`/`bodyless`/`judgeless`/`vacuities` will already be
  exhaustive matches by the time this lands — named explicitly as needing
  a `When` arm too, not just `target`/`documented_field`/`addressed_field`).
  Bundled in the same entry: `Shape::LeadingDotSlash`, decision 0041's
  second, smaller addition, since it shares contract.rs. (3) "SDK: … gains
  when(guard, clauses); schema/lock round-trip" — filed
  CONTRACT-WHEN-SDK-AUTHORING-ROUNDTRIP (blockedBy the engine entry — ts-rs
  generates ClauseRow.ts from the Rust struct, so the SDK round-trip test
  needs the Rust decode side settled first). (4) "marketplaceDefaultContract
  completes the source union… retires its header hold; the other two
  shipped contracts… re-examined" — investigated sdk/src/builtins.ts and
  found exactly three vocabulary-shaped holds (marketplace's source union,
  mcp-server's transport-conditional fields, hook's per-event schema — no
  fourth candidate; every other shipped contract's absence is explicitly
  semantic). Filed one entry, BUILTINS-DEFAULT-CONTRACT-HOLDS-CLOSE
  (blockedBy the SDK entry), covering all three since they're one
  Consequences bullet in one file. Per investigation, hook's hold is an
  addressing-reach gap (the collection address never walks into the
  nested matcher-group array) that `when`/`enum`/`type` cannot close —
  the entry asks build to confirm this on disk and reword the hold rather
  than presuming closure. (5) "The friction capture's residue is fully
  routed; no fork record remains" — verified: open-questions.md carries
  no fork about the discriminated-union predicate or the source union;
  nothing to close. Three new chained entries filed; zero forks touched.
- Queue: 33 pending — 3 pickable OPEN (unchanged: DRIFT-INCLUDE-SOURCE-PATH-CWD-LEAK,
  BUILTIN-KIND-DEFINITIONS-RESULT-COLLAPSE, JSON-MANIFEST-TOP-LEVEL-OBJECT-PARSE-CONSOLIDATE
  — pairwise file-disjoint, re-checked this tick), 28 chained blockedBy (25
  prior + 3 new this tick, all resolving to live tags), 2 parked on human
  action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — untouched).
  Open forks: (multi-harness-projection), (lazy-grounds) unchanged, neither
  touched by this tick's derivation. Refactor captures: 0 live. Friction: 0
  live. Inbox: 0 notes (job not serviced this tick; untouched, still
  empty). Disjointness re-checked: no two OPEN entries share a file; 33
  unique tags, no duplicates; every blockedBy tag resolves (validated by
  script).

Plan continues: yes — the posture sweep is still mid-rotation (judges
next) and no commit past 73c76ca has landed to re-trigger reconciliation,
so the sweep is the next live input.
