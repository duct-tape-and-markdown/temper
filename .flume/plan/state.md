# Plan state

- **Phase:** reconcile. HEAD a5d7950.
- **Last shipped:** **CONTRACT-DIR** (`build` 375a1cc / `chore` a5d7950) — the
  byte-parity corpus promoted into a shared top-level `contract/` dir (`manifest/`
  + `schema/` goldens + README), backed by Rust-first schema gen (schemars +
  ts-rs, Cargo.toml). Verified on disk; build pruned it from the queue. **The
  ratified SDK wave has fully drained** — no in-fence ratification item remains.
- **This tick:** reconciled against disk. CONTRACT-DIR shipped and was already
  pruned. All three carried entries hold, premises re-checked on disk:
  EXTRACTION-VOCAB-GAPS + AGENT-KIND deferred (no consumer — `Primitive::Field`
  flat-reads `unit.frontmatter.get(key)` at kind.rs:836; `kinds/claude-code/`
  carries only memory/rule/skill, no `agent/`), PACKAGING-CHANNELS parked (root
  package.json still private `temper-flume-harness`). Confirmed no hidden drift
  from `(extraction-source-not-mechanism)`: `skill_features`/`rule_features` are
  now thin `features(&definition(..), unit)` wrappers over the one generic
  composed path (builtin_kind.rs:163) — the intended end state, not scaffolding
  to retire. Inbox empty.
- **Pickable now:** **none** — the queue is deferred/deferred/parked only. No
  `open` head; the in-fence work has drained.
- **What's next:** the remaining work is human, not a pending entry — the dogfood
  migration onto the SDK authoring face (ledger's TS-primary ceremony) and
  PACKAGING-CHANNELS's release creds. No-demolition holds (D6): engine-side
  retirements stay evidence-gated behind that migration.

Plan continues: no — queue reconciled and truthful, inbox empty, and no pickable
`open` head remains (all entries deferred/parked). Remaining work needs a human
ceremony, not a build tick; nothing to hand to build.
