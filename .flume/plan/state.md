# Plan state

- **Phase:** residue sweep + inbox drain. Spec-delta empty (no `specs/` commit
  since 7a3ff54); the only commit since is one flume chore (488e685). Intent
  unmoved — but the inbox carried John's pre-v0.1 residue routing, and the sweep
  ran regardless.
- **Last shipped:** TEMPER-TOML-ZERO (build 4d6e813 / chore ed95bcc), terminal of
  the S1→S7 `(inplace-lock-producer)` demolition. Re-verified on disk this tick:
  the gate composes each kind's contract from the lock's `ClauseRow`; `rg
  temper.toml src/` = 0.
- **This tick:** drained John's inbox routing (the three named demolitions +
  register `(authority-home)`) and swept the corpus residue against `src/`.
  Verified on disk: the **package noun** is live (`PackageResolver`,
  `Requirement.package`, `Membership.source_package`/`conforms_to` in compose.rs;
  the roster conformance pass; `RequirementRow.package`/`MembershipRow.source_package`
  columns in drift.rs; the dead `RequirementRow.package` type in sdk); the
  **reachability dial** is live (`reachability_from_declarations` main.rs:1188,
  the opt-in block at :749, the SDK `Harness.reachability` emitted at
  declarations.ts:152); the **requirement facet spelling** (count/unique/membership/degree)
  is live in compose.rs + drift.rs + roster.rs + sdk/contract.ts. Filed the
  package chain (**PKG-NOUN-ENGINE-RETIRE** open → **PKG-NOUN-LOCK-ROWS** →
  **PKG-NOUN-SDK-COLUMN**, serial on the shared drift/main spine) and
  **REACH-DIAL-RETIRE** (chain tail). Parked **REQUIREMENT-CLAUSES-RECUT** for a
  decomposition ceremony — the inbox's "disjoint sdk entry" framing under-scopes
  it (the Rust facets + SEAM bump make it a serialized chain). Registered
  `(authority-home)` (the SDK's hardcoded `authority=shared` fact vs the
  note/warn/block vocabulary; no home in the four-field assembly — needs John).
- **In flight:** 7 entries. Pickable now (disjoint files, parallel-safe):
  **KIND-BUILTIN-CONST-RETIRE** (src/kind.rs) and **PKG-NOUN-ENGINE-RETIRE**
  (compose/roster/main). Then the serial chain PKG-NOUN-LOCK-ROWS (drift) →
  PKG-NOUN-SDK-COLUMN (sdk) → REACH-DIAL-RETIRE (main+sdk). Parked:
  REQUIREMENT-CLAUSES-RECUT (ceremony) and PACKAGING-CHANNELS (release creds).
- **What's next (human-gated):** the REQUIREMENT-CLAUSES-RECUT decomposition
  ceremony; `(authority-home)` ruling; the dead `kinds/`+`packages/` product tree
  deletion (10-contracts residue, but fence-excluded — a human `chore`, not
  buildable; build.rs already embeds nothing); PACKAGING-CHANNELS release setup +
  USPTO screen; the genre-fence-format workshop (cascade pilot); the standing OPEN
  forks needing John.

Plan continues: no — the sweep produced two pickable `open` entries
(KIND-BUILTIN-CONST-RETIRE, PKG-NOUN-ENGINE-RETIRE) plus a serial demolition
chain behind them; the inbox is drained and the rest is human-gated. Hand to
build; the queue drains by building, not re-planning.
