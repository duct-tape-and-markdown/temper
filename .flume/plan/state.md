# Plan state

- **Phase:** reconcile. HEAD 1c33491.
- **Last shipped:** the SDK altitude wave (byte-parity → body-resolution →
  projection-lock, all on disk); then the interactive session **ratified 'the
  SDK is the product'** (`specs:` 71d0d30) and routed the reconcile
  (`chore(flume)` 1c33491 — contract/** entered build's fence).
- **This tick:** drained the inbox and **filed the ratification's three
  in-fence work items** as a serialized chain — **SDK-EMIT-REFUSALS** (open) →
  **SDK-ASSEMBLY-ARTIFACTS** (blockedBy) → **CONTRACT-DIR** (blockedBy). They
  serialize because SDK-EMIT-REFUSALS/SDK-ASSEMBLY-ARTIFACTS both edit
  `sdk/src/emit.ts` and CONTRACT-DIR shares `sdk/test/emit.test.ts`. The three
  carried entries (EXTRACTION-VOCAB-GAPS, AGENT-KIND deferred; PACKAGING-CHANNELS
  parked) reconciled against disk and stay accurate (`Primitive::Field` flat-reads
  kind.rs:836; no `kinds/claude-code/agent/`; root package.json still the private
  flume manifest). Second inbox line (toml_edit style instability) → accepted debt
  + a standing-discipline note in open-questions.
- **Pickable now:** **SDK-EMIT-REFUSALS** — sdk/** only, in-fence, clean cite
  (20-surface "Emit refuses before it writes"). Build has a head to pick.
- **What's next:** build drains the chain one tick at a time (each ship un-blocks
  the next on the following reconcile). No-demolition holds (D6): engine-side
  retirements stay evidence-gated behind the human dogfood migration, never
  filed. The genre projection carrier stays a NAMED OPEN; the dogfood migration
  (temper.config.ts) is a human ceremony, not an entry.

Plan continues: no — inbox drained, queue reconciled and truthful, and a
pickable `open` head (SDK-EMIT-REFUSALS) now exists. Building is how the queue
drains; hand to build.
