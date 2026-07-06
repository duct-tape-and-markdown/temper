# Plan state

- **Phase:** residue sweep + gate re-test. Spec-delta empty (no `specs/` commit
  since 7a3ff54); commits since last plan tick are two `build:` + one
  `chore(flume)` shipping the package-noun and dead-const cuts. Intent unmoved —
  the sweep ran regardless and unblocked the next chain link.
- **Last shipped:** KIND-BUILTIN-CONST-RETIRE (93dc3dc) and PKG-NOUN-ENGINE-RETIRE
  (5ed652f), both dropped from pending by chore 64be4a0. Re-verified on disk:
  `kind::BUILTIN_KINDS` is gone from src/kind.rs; compose.rs `Requirement` and
  roster.rs carry no `package`/`conforms_to`/`source_package` facet.
- **This tick:** PKG-NOUN-ENGINE-RETIRE shipping unblocked the lock cut, so I
  reconciled the package chain. Discovered on disk that the requirement `package`
  facet lives in a **second** Rust home the old queue missed — `document.rs`'s
  `PublishedRequirement.package` (parsed + emitted in the member-document codec,
  no longer lifted into `compose::Requirement`). It shares tests/requirement_roster.rs
  with the drift-lock cut (that file carries RequirementRow, PublishedRequirement,
  AND MembershipRow literals), so the two Rust homes **cannot** be parallel `open`s.
  Folded the former **PKG-NOUN-LOCK-ROWS** into **PKG-NOUN-REQ-FACET** (drift.rs +
  document.rs + the stale main.rs:1032 comment + 7 test files), now `open`.
- **In flight:** 5 entries. Pickable now: **PKG-NOUN-REQ-FACET** (open, the one
  disjoint slice this tick). Serial chain behind it: PKG-NOUN-SDK-COLUMN (drop the
  dead SDK `RequirementRow.package`, blockedBy REQ-FACET) → REACH-DIAL-RETIRE
  (retire the assembly reachability dial across main.rs + sdk, blockedBy
  SDK-COLUMN — line numbers re-verified on disk: `reachability_from_declarations`
  main.rs:1141, opt-in block :726-752, SDK `Harness.reachability` assembly.ts:35,
  emission declarations.ts:152, tests/reachable_gate.rs retired). Parked:
  REQUIREMENT-CLAUSES-RECUT (decomposition ceremony) and PACKAGING-CHANNELS
  (release creds).
- **What's next (human-gated):** the REQUIREMENT-CLAUSES-RECUT decomposition
  ceremony (count/unique/membership/degree → `clauses` array, cross-language +
  SEAM bump); the `(authority-home)` ruling (SDK hardcodes `authority=shared`,
  declarations.ts:151); the dead `kinds/`+`packages/` product-tree deletion
  (fence-excluded, a human `chore`); PACKAGING-CHANNELS release setup + USPTO
  screen; the genre-fence-format workshop (cascade pilot). Inbox empty.

Plan continues: no — the sweep reconciled the queue against disk (ENGINE-RETIRE
shipped, document.rs residue folded in) and left one pickable `open` entry with a
serial chain behind it; the inbox is drained and the rest is human-gated. Hand to
build; the queue drains by building, not re-planning.
