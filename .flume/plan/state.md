# Plan state

- **Phase:** reconcile. HEAD 47cc021.
- **Last shipped:** **SDK-PROJECTION-LOCK** — the SDK altitude wave's last link
  (`build` 8796ce8, removed by `chore(flume)` 47cc021). Verified on disk:
  `sdk/src/project.ts` writes the byte-faithful `.claude/**` projection,
  `sdk/src/lock.ts` stamps `source_hash`/`emit_hash`, both exported from
  `sdk/src/index.ts`; `emit.ts` carries `resolveBody` (fromFile + mention
  resolution-check). The TS authoring face now emits manifest + projection +
  lock in one deterministic `emit`. **The SDK altitude wave is closed** (all
  three links — byte-parity, body-resolution, projection-lock — shipped).
- **This tick:** no queue change — reconciled the three carried entries against
  disk and all stay accurate (EXTRACTION-VOCAB-GAPS: `Primitive::Field`
  still flat-reads `unit.frontmatter.get(key)` at kind.rs:836, no nested-key
  consumer; AGENT-KIND: no `kinds/claude-code/agent/` on disk; PACKAGING-CHANNELS:
  root `package.json` still the private `temper-flume-harness` flume manifest).
  Refreshed this stale `state.md` and marked the altitude wave fully drained in
  the `(scripted-altitude-reconcile)` fork tail. Inbox empty — nothing to drain.
- **Pickable now:** **none.** Every pending entry is deferred (no consumer:
  EXTRACTION-VOCAB-GAPS, AGENT-KIND) or parked on human creds
  (PACKAGING-CHANNELS). No `open` entry — build has nothing to pick this tick.
- **What's next:** the pipeline waits on **human forks**, not more engine work.
  The SDK/floor/front-door waves have all drained (AGENTS.md, CHANGELOG.md,
  CONTRIBUTING.md, dual LICENSE all on disk). Open human decisions: the ledger's
  TS-primary reformulation ruling (unblocks the dogfood's migration onto the SDK
  authoring face), PACKAGING-CHANNELS release creds, the genre-adoption pilot
  (a staged human ceremony), and the two OPEN strategic forks
  (`edge-representation-unify` join→graph, `default-assembly-as-data`) that each
  need a spec Decision before any dependent can be filed.

Plan continues: no — queue reconciled and truthful, inbox empty, no new gap
carries a clean `per` cite (remaining gaps are human-gated forks or need a spec
Decision first). Nothing pickable to hand build; the pipeline is blocked on
human forks, not on further plan work. Re-planning an unchanged queue would be
the failure mode.
