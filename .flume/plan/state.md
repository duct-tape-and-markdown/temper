# Plan state

- **Phase:** reconciled DRIFT-DIFF-RETIRE shipped against src/. Queue now
  quiescent — one parked entry, nothing autonomously pickable.
- **Last shipped:** DRIFT-DIFF-RETIRE (build b9c8f3c / chore 028bd72) — the
  dead four-state drift differ is gone from src/drift.rs (verified on disk:
  the `diff`/`classify`/`DriftState`/`DriftReport`/`SurfaceArtifact` family
  removed; only the emit-report renderers `render_field`/`render_emit`
  survive, zero usages of the retired types).
- **This tick:** verified the retirement on disk and dropped it from the
  active reasoning; re-checked the two other ledger-named "small" cuts and
  found neither decoupled — the kind-blind read path is `(builtin-workspace-
  qualified-key)`/demolition-entangled, and mentions→citations is empty by
  the main.rs:507-510 comment "until an altitude serializes mentions" (the
  human-gated SDK-primary altitude). Inbox empty; nothing to drain.
- **In flight:** none pickable. PACKAGING-CHANNELS stays parked (needs npm
  org + @temper scope, marketplace/signing creds; rides the SDK-primary
  foundation).
- **What's next:** John's hand — the demolition core (copy-tree gate rewrite +
  producer/codec/AuthorLayer cuts) rides the SDK-primary front door with
  `init`'s re-shape; the corpus shadow of the dogfood deactivation; release
  creds. No autonomously-derivable green chain remains.

Plan continues: no — the queue is reconciled (DRIFT-DIFF-RETIRE dropped as
shipped), the inbox is empty, and every remaining thread is human-gated on the
SDK-primary front door. Nothing pickable to hand build; the loop is quiescent
until John's hand moves.
