# Plan state

- Spec derived through: abe5d5d
- Audited through: b745415
- Residue swept through: b745415
- This tick: DRAINED the inbox — the 07-16 centercode field report (filed
  9adefde) on the render-hook parameter types missing from the package root.
  Routed as one entry plus one fork; the note is removed.
  **Gap re-verified on disk at 420da04, and it grew.** The note stamped
  `observed at 58efe11`, so the premise was diffed forward over the three
  `sdk/src/` commits since (91c288c, 5c3a618, 28ad61f): the hole is unmoved.
  But the note reported a *pair* and asked for a *closure*, and the closure is
  six types, not two — `ResolvedEmbeddedMemberValue`, `EdgeTargetFacts`,
  `ResolvedEmbeddedMemberCollectionEntry` (the reported kin), plus
  `KindOptions` (`kind()`'s own second parameter), `Template` (named by the
  root-exported `KindFacts.templates`), and `SettingsResidue` (named by the
  root-exported `EmitResult.settings`). Scoped to the verified gap, never the
  reported one, per job 1.
  **The sweep found a seventh hole the report did not name:** `globValid`
  (contract.ts:116) is the only one of 24 predicate constructors absent from
  the root — reachable to `builtins.ts` alone (503/595), so an author cannot
  spell it over their own kind. The vocabulary is closed and the enum is the
  authority (`contract.md`, "clause"); a 23/24-wide author face disagrees with
  it silently. Same face, same file, one entry: SDK-ROOT-EXPORT-CLOSURE.
  **One fork registered, not papered over.** The closure rule collides with
  `pipeline.md`'s "the payload is internal … not a designed public
  interchange": applying it to the root-exported `Declarations` would promote
  ~28 generated row types to public API. `(seam-rows-public-face)` carries the
  collision, plan's position (retract, don't complete), and the objection that
  must answer it. The entry ships the authoring half only — which neither
  reading contests — so the fork blocks nothing.
  Closing checklist: `per` cite verified in-file; every symbol/line claim
  re-verified on disk; field lengths validated (summary 166/200, notes
  465/500). The new entry is `open` and file-disjoint from all four standing
  entries (`sdk/src/index.ts`, `sdk/test/emit.test.ts`,
  `sdk/test/root_exports.test.ts` appear in no other). Cursors copied forward
  verbatim — this tick derived no spec and audited no window.
- Queue: 1 pickable (SDK-ROOT-EXPORT-CLOSURE); three gated in two chains, and
  **two of those gates are now stale** — SKILL-NESTED-REFERENCE-DOCS and
  SUPPORTING-DOC-REACH-CLAUSE name NESTED-FILE-DISCOVERY /
  PREDICATE-SELECTION-ALGEBRA, both shipped at 420da04. Re-testing them is next
  tick's audit, not this tick's job; the refs gate does not read `blockedBy`,
  so nothing reverts and no build wave can pick a stale-gated entry.
  PACKAGING-CHANNELS-REMAINDER parked (Apple notarizing + v0.1 tag).

Plan continues: yes — post-ship reconciliation of the b745415..420da04 window
(5c3a618, 2722ca2, 28ad61f: PLACEMENT-KEY-NUL-DELIMITER, NESTED-FILE-DISCOVERY,
PREDICATE-SELECTION-ALGEBRA), both code cursors trailing it. The audit owes the
two stale gates above a re-test and the work behind them a derivation.
