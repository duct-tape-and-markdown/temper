# Plan state

- **Phase:** residue sweep + gate re-test. Spec-delta empty (no `specs/` commit
  since 7a3ff54); the only new commits are the PKG-NOUN-REQ-FACET ship (one
  `build:` + one `chore(flume)`). Intent unmoved — the sweep ran regardless and
  unblocked the next chain link.
- **Last shipped:** PKG-NOUN-REQ-FACET (bc393f5 / chore c253231) — the lock+codec
  tail of the package-noun retirement. Re-verified on disk: drift.rs carries no
  `RequirementRow.package`/`MembershipRow.source_package`, document.rs no
  `PublishedRequirement.package`. Requirement-side package residue in Rust is
  fully cut (no `conforms_to`/`source_package` in src/).
- **This tick:** PKG-NOUN-REQ-FACET shipping unblocked its dependent. Reconciled
  on disk: the dead SDK `RequirementRow.package` still sits at
  declarations.ts:39 (never written — `requirementRows` maps only
  name/kind/required/verified_by, :137-141; no `Requirement` in contract.ts feeds
  it). Flipped **PKG-NOUN-SDK-COLUMN** from `blockedBy PKG-NOUN-REQ-FACET` to
  `open`. The reachability-dial symbols re-verified present
  (`reachability_from_declarations` main.rs:1141, opt-in main.rs:726-747, SDK
  `Harness.reachability` assembly.ts:35, emission declarations.ts:152,
  tests/reachable_gate.rs). No new residue fileable: `packages/`+`kinds/` trees
  and `read::bound_package` are the kept-on-purpose floor-embedding foundation
  (human/SDK-primary-gated), not this noun's residue.
- **In flight:** 4 entries. Pickable now: **PKG-NOUN-SDK-COLUMN** (open, the one
  disjoint sdk/ slice). Serial chain behind it: REACH-DIAL-RETIRE (retire the
  assembly reachability dial across main.rs + sdk, blockedBy SDK-COLUMN — shares
  declarations.ts + assembly.ts). Parked: REQUIREMENT-CLAUSES-RECUT (decomposition
  ceremony) and PACKAGING-CHANNELS (release creds).
- **What's next (human-gated):** the REQUIREMENT-CLAUSES-RECUT decomposition
  ceremony (count/unique/membership/degree → `clauses` array, cross-language +
  SEAM bump), which also folds in reachability-as-edge-scope-clause; the
  `(authority-home)` ruling (SDK hardcodes `authority=shared`,
  declarations.ts:151); the dead `kinds/`+`packages/` product-tree deletion
  (fence-excluded, a human `chore`); PACKAGING-CHANNELS release setup + USPTO
  screen; the genre-fence-format workshop (cascade pilot). Inbox empty.

Plan continues: no — the sweep reconciled the queue against disk, unblocked
PKG-NOUN-SDK-COLUMN, and left one pickable `open` entry with a serial chain
behind it; inbox drained, no fork moved, the rest is human-gated. Hand to build;
the queue drains by building, not re-planning.
