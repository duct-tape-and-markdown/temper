# Plan state

- **Phase:** reconcile. HEAD 7c62747.
- **Last shipped:** **SDK-ASSEMBLY-ARTIFACTS** (`build` 50ed952 / `chore`
  7c62747) — the assembly's locus-less facts (bindings, roster) emitted as small
  temper-owned artifacts (`sdk/src/assembly_artifacts.ts` + test), verified on
  disk; build pruned it from the queue.
- **This tick:** reconciled against disk. SDK-ASSEMBLY-ARTIFACTS shipped, so its
  only dependent **CONTRACT-DIR flips blockedBy→open** — the last in-fence
  ratification item, emit surface now frozen. Premises re-checked: no `contract/`
  dir, no schemars/ts-rs in Cargo.toml, ManifestMember at compose.rs:124,
  schema.rs `emit` path present. The three carried entries hold: EXTRACTION-VOCAB-GAPS
  + AGENT-KIND deferred (no consumer — `Primitive::Field` flat-reads kind.rs:836;
  no `kinds/claude-code/agent/`), PACKAGING-CHANNELS parked (root package.json
  still `temper-flume-harness`, private). Inbox empty.
- **Pickable now:** **CONTRACT-DIR** — the sole open head; `contract/` + Rust
  goldens + Rust-first schema gen (schemars/ts-rs). Fully in-fence.
- **What's next:** build ships CONTRACT-DIR and the ratified SDK wave fully
  drains. Remaining queue is then deferred/parked only — NEXT is the human dogfood
  migration onto the SDK (ledger's TS-primary ceremony), not a pending entry.
  No-demolition holds (D6): engine-side retirements stay evidence-gated behind
  that migration.

Plan continues: no — queue reconciled and truthful, inbox empty, and a pickable
`open` head (CONTRACT-DIR) exists. Building is how the queue drains; hand to build.
