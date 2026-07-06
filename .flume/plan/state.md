# Plan state

- **Phase:** unblock + rewrite. Spec-delta empty (no `specs/` commit since
  e227a27); inbox empty. The only new commits are the PKG-NOUN-SDK-COLUMN ship
  (one `build:` 13d5da6 + one `chore(flume)` 534828d). Intent unmoved — the
  residue sweep + gate re-test ran regardless and unblocked the next chain link.
- **Last shipped:** PKG-NOUN-SDK-COLUMN (13d5da6 / chore 534828d) — dropped the
  dead `RequirementRow.package` field from the SDK. Re-verified on disk:
  `rg .package sdk/src/{declarations,contract}.ts` returns nothing; the
  package-noun's SDK type-surface residue is fully cut.
- **This tick:** its ship unblocked the dependent. Re-verified the dial symbols
  on disk (`reachability_from_declarations` main.rs:1141, opt-in main.rs:726-747,
  SDK `Harness.reachability` assembly.ts:35, emission declarations.ts:151-152,
  tests/reachable_gate.rs), then **rewrote REACH-DIAL-RETIRE**: flipped
  `blockedBy PKG-NOUN-SDK-COLUMN` → `open`; refreshed the SDK line numbers the
  one-line SDK-COLUMN deletion shifted; **added src/drift.rs** to sweep four
  stale `AssemblyFactRow` doc comments naming `reachability` as a fact
  discriminator (a real blast-radius gap); and replaced the old
  `fact.*reachability` acceptance regex, which false-matched surviving
  reachability-*concept* prose (kind.ts:24, the drift docs), with a precise
  dial-symbol check. Reachability-the-predicate (`pub fn graph::reachable`,
  tests/graph.rs) is kept — only the assembly dial retires.
- **In flight:** 3 entries. Pickable now: **REACH-DIAL-RETIRE** (open, one
  disjoint slice over main.rs + sdk + drift.rs + a test retire; no other open
  entry touches those files). Parked: REQUIREMENT-CLAUSES-RECUT (decomposition
  ceremony — its four facets re-verified live in contract.ts:83-86 / compose.rs
  / drift.rs / roster.rs) and PACKAGING-CHANNELS (release creds).
- **What's next (human-gated):** the REQUIREMENT-CLAUSES-RECUT decomposition
  ceremony (count/unique/membership/degree → `clauses` array, cross-language +
  SEAM bump), which folds in reachability-as-edge-scope-clause (the dial's
  corpus replacement, deferred behind this); the `(authority-home)` ruling (SDK
  hardcodes `authority=shared`, declarations.ts:150); PACKAGING-CHANNELS release
  setup + USPTO screen; the genre-fence-format workshop (cascade pilot). Inbox
  empty; open-questions unmoved (no fork resolved).

Plan continues: no — the sweep unblocked and rewrote REACH-DIAL-RETIRE, leaving
one pickable `open` slice; inbox drained, spec-delta empty, no fork moved, the
rest human-gated. Hand to build; the queue drains by building, not re-planning.
