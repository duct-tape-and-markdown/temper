# Plan state

- Spec derived through: f7d870c
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: model done — formats next
- This tick: POSTURE SWEEP. Nothing live above it: inbox/refactor-captures
  empty, no specs/ commits past f7d870c, no src/sdk/src/tests/ commits past
  60faee0 (`git log 60faee0..HEAD -- src/ sdk/src/ tests/` empty). Rotation
  resumed at `model` (`kind`, `contract`, `compose`, `schema`, `roster` per
  architecture.md's codemap) — swept via a read-only agent pass over the
  five files against every engineering.md lens plus the architecture
  invariants, then every candidate finding re-verified myself on disk
  (symbol, line, grep for consumers) per the posture-sweep rule's "verified
  on disk this tick" bar before filing. Two lenses turned up nothing
  (libraries-before-hand-rolls, cost-hoisting, green-verdict-non-vacuous);
  one candidate (gauntlet-corpus coverage of contract.rs's node-set
  predicates) was discarded on closer read — gauntlet.rs's own doc names
  its job as the four *structural composition* seams, not general
  predicate coverage, so the absence isn't residue against that section.
  Four findings held up and were filed:
  - `roster.rs` (model) imports `crate::builtin` (provider) solely for
    `kind_narrowing_clause`, a generic clause-builder with no Claude-Code
    data — architecture.md's literal invariant ("the provider face is
    ... never a dependency of the model") — → filed
    `ROSTER-BUILTIN-KIND-NARROWING-RELOCATE` (open, disjoint: touches
    builtin.rs/compose.rs/roster.rs/tests/contract_template.rs, none
    shared with any other entry). Noted but not fixed: roster.rs's
    separate `crate::engine` import is roster's own doc-justified
    delegation to the judge algebra, and architecture.md states no
    model-never-depends-on-judges rule — surfaced in the entry's notes
    for a human read, not filed as residue.
  - `kind.rs` ships three zero-consumer `pub` items (`Commitment::label`,
    `Content::label`, `CustomKind::qualified_name` — grep-verified no
    caller outside kind.rs's own inline tests) — engineering.md, "An
    export earns its consumer" → filed `KIND-ZERO-CONSUMER-EXPORTS-PRUNE`.
  - `contract.rs`'s `declared_keys` uses a `_ => None` wildcard where its
    two neighbors (`target`, `documented_field`) over the same `Predicate`
    enum exhaustively name every variant — engineering.md, "A shared
    concept is one type" → filed `CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH`.
  - `Predicate::RequireSections` ships a half-built round trip: the SDK
    `requireSections()` takes no `sections` argument (unlike
    `dependencyExists`, whose absent constructor is explicitly documented
    as a deliberate hold, this one names no such thing), `ClauseRow` has
    no column for it, and `predicate_from_row` has no decode arm — yet
    `engine.rs` fully implements and tests its judging with hand-built
    Rust data. Verified end to end (contract.ts, declarations.ts's
    `clauseRow` mapper, drift.rs's `ClauseRow` struct, contract.rs's
    `predicate_from_row`) before filing → `CONTRACT-REQUIRE-SECTIONS-
    ROUNDTRIP`, cited to pipeline.md's "The lock" (a declaration row
    family that cannot carry its own predicate's argument).
  Chain safety (pending-entry.md, "Disjoint, or serialized"): the last
  three new entries all eventually touch a file the existing
  DISCOVERY→...→EXTRACT-FOUNDATION-BOUNDARY-RESTORE chain touches
  (kind.rs, contract.rs transitively) or each other (contract.rs,
  drift.rs), and the schema's single-tag `blockedBy` can't express two
  independent priors, so all three chain linearly behind
  EXTRACT-FOUNDATION-BOUNDARY-RESTORE: `KIND-ZERO-CONSUMER-EXPORTS-PRUNE`
  → `CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH` →
  `CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP`. `ROSTER-BUILTIN-KIND-NARROWING-
  RELOCATE` shares no file with anything else queued, so it stays `open`
  alongside the two already-open entries.
- Queue: 12 pending — 3 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-
  COLLAPSE, FRONTMATTER-TEST-SYNTHETIC-KINDS, ROSTER-BUILTIN-KIND-
  NARROWING-RELOCATE; all disjoint files), 7 chained blockedBy
  (DRIFT-LOCK-ROW-WALK-CONSOLIDATION → DRIFT-EMIT-LOCK-PARSE-HOIST →
  PLACEMENT-MODULE-EXTRACTION → EXTRACT-FOUNDATION-BOUNDARY-RESTORE →
  KIND-ZERO-CONSUMER-EXPORTS-PRUNE → CONTRACT-DECLARED-KEYS-EXHAUSTIVE-
  MATCH → CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. No live refactor
  captures; inbox empty.

Plan continues: yes — posture sweep resumes at `formats`
(`frontmatter`/`document`/`json_manifest`/`toml_document`), the roster's
next subsystem, once nothing above it is live.
