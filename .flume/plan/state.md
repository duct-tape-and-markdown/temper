# Plan state

- **Phase:** reconcile. HEAD d6ce2c2.
- **Last shipped:** MEMORY-PROJECTION-SDK (`build` 1ec14fb / `chore` d6ce2c2) —
  SDK emit now projects a module-carried memory to its root `CLAUDE.md`/`AGENTS.md`
  locus; verified on disk (`isProjectedKind` returns `memory`, `sdk/src/project.ts:53`).
- **This tick:** inbox empty. Re-verified the three carried entries hold, premises
  on disk unchanged: EXTRACTION-VOCAB-GAPS deferred (`Primitive::Field` still
  flat-reads `unit.frontmatter.get(key)`, kind.rs:835 — no nested-key consumer),
  AGENT-KIND deferred (`BUILTIN_KINDS = ["skill","rule"]`, kind.rs:30; no
  `kinds/claude-code/agent/`), PACKAGING-CHANNELS parked (only `temper.yml` CI —
  no `release.yml`; root `package.json` still private `temper-flume-harness`).
  Re-cut the SDK-seams fork header: the memory slice these four seams shared a file
  with has shipped, so they no longer name an open head — each awaits a human ruling.
- **Pickable now:** none. No `open` head — the three carried entries are
  deferred/parked; the four SDK seam forks (`sdk-placement-round-through`,
  `gate-kind-spelling-and-unknown-kind`, `gate-reads-assembly-artifacts`,
  `module-carriage-manifest-shape`) each need a human ruling and share
  `sdk/src/{emit,project,manifest}.ts`, so none can be filed as a parallel `open` entry.
- **What's next:** a human ruling on one of the four SDK seam forks turns it into
  the next serialized SDK entry; the dogfood's full migration onto the SDK is the
  ledger's TS-primary human ceremony (not a pending entry). Until then the queue is
  drained and blocked on human input.

Plan continues: no — queue reconciled, inbox drained, memory slice shipped and its
fork header re-cut. No `open` head remains: the next move is a human ruling on the
SDK seam forks, not more plan or build work.
