# Plan state

- **Phase:** reconcile. HEAD e1a1fb5.
- **Last shipped:** both SDK-seam chain tails — GATE-READS-ASSEMBLY (`build`
  ed97588) and SDK-PLACEMENT-ROUNDTHROUGH (`build` 2377f4e), shipped together
  (`chore` e1a1fb5). Verified on disk: `src/assembly_artifacts.rs` + `main.rs:853`
  `assembly_artifacts::load` reads roster.toml/bindings.toml as the gate's
  assembly source; `sdk/src/project.ts:250` threads `committedPlacements` through
  `projectBytes`. With these, **all four ruled SDK seams are closed.**
- **This tick:** confirmed both tails landed and are already dropped from the
  queue. No corpus↔src drift to file — the queue is fully reconciled. Inbox empty.
  The three carried entries all still hold on disk: EXTRACTION-VOCAB-GAPS deferred
  (kind.rs:836 still flat-reads `frontmatter.get(key)`, no consumer), AGENT-KIND
  deferred (no `agent` kind under kinds/claude-code/, no AGENT_PACKAGE), and
  PACKAGING-CHANNELS parked (package.json still the private flume manifest).
- **Pickable now:** none. Every remaining entry is deferred or parked — no `open`
  entry exists. The pipeline is idle pending human action.
- **What's next (human, not a pending entry):** the dogfood's full migration onto
  the SDK is the ledger's TS-primary reformulation ceremony (session + John).
  Reviving any carried entry also needs a human (a consumer for
  EXTRACTION-VOCAB-GAPS / AGENT-KIND; npm creds for PACKAGING-CHANNELS).

Plan continues: no — queue reconciled, no `open` work remains; forward motion
now needs a human ceremony, not a build tick.
