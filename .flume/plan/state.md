# Plan state

- **Phase:** residue sweep. Spec-delta empty (no `specs/` commit since e227a27;
  93c1a30..HEAD carries no `specs/` change); inbox empty. New commits since last
  tick: the REACH-DIAL-RETIRE ship (build 207e701 + chore dc670f6). Intent
  unmoved — the sweep runs regardless and found one un-derived residue slice.
- **Last shipped:** REACH-DIAL-RETIRE (207e701 / chore dc670f6) — retired the
  assembly reachability dial into the clause algebra. Re-verified on disk: no
  `reachability_from_declarations`, no SDK `Harness.reachability` field (only
  kept-concept prose in main.rs/kind.ts); the `reachable` predicate
  (graph.rs:370) is kept. Done, correctly out of pending.
- **This tick:** the mandatory residue sweep over the package-noun retirement
  (10-contracts, the package-noun Decision). Verified the conformance pass,
  `conforms_to`, and `RequirementRow.package` facet are all gone (shipped by the
  PKG-NOUN-* wave). The **one** un-derived residue: the Decision's named
  "diagnostics that teach the package vocabulary" — read.rs:412's `explain`
  output still says "Governing package … binds the `X` package", and read.rs +
  builtin.rs cite two spec sections the re-cut deleted (verified absent from
  specs/). Filed as **PKG-NOUN-EXPLAIN-DIAGNOSTIC** (open).
- **In flight:** 3 entries. Pickable now: **PKG-NOUN-EXPLAIN-DIAGNOSTIC** (open,
  disjoint — read.rs diagnostic + builtin.rs cite + 5 `read_verbs__why_*.snap`;
  neither parked entry touches those). Parked: REQUIREMENT-CLAUSES-RECUT
  (decomposition ceremony — its four facets re-verified live in contract.ts:83-86
  / compose.rs CountBound:105 / Membership:158 / DegreeBound:118) and
  PACKAGING-CHANNELS (release creds).
- **What's next (human-gated):** the REQUIREMENT-CLAUSES-RECUT decomposition
  ceremony (count/unique/membership/degree → `clauses` array, cross-language +
  SEAM bump), which folds in reachability-as-edge-scope-clause (the dial's corpus
  replacement, deferred behind it); the `(authority-home)` ruling (SDK hardcodes
  `authority=shared`, declarations.ts:151); PACKAGING-CHANNELS release setup +
  USPTO screen; the genre-fence-format workshop (cascade pilot). Inbox empty;
  open-questions unmoved (no fork resolved).

Plan continues: no — the sweep filed one pickable `open` slice
(PKG-NOUN-EXPLAIN-DIAGNOSTIC); the rest is verified human-gated (two ceremonies +
release creds). Inbox drained, spec-delta empty, no fork moved. Hand to build; the
queue drains by building, not re-planning.
