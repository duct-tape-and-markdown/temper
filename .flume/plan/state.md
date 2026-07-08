# Plan state

- Spec derived through: 474835b
- Audited through: a112dbe
- Residue swept through: 91fb466
- This tick: Inbox (job 1) — two field reports drained. (1) SDK version-skew:
  verified live — `sdk/prose.ts` file() went two-arg (moduleUrl) in 78a48db,
  after the published `@dtmd/temper@0.0.4` tarball (still one-arg, confirmed
  via `npm pack`); filed SDK-VERSION-LOCKSTEP to derive install.rs's pin from
  sdk/package.json instead of a hand-typed constant, plus bump to 0.0.5.
  (2) 0017 no-middles sweep, three routes: dead `Template.leaves/collections`
  + dead `import_hash` provenance fallback (both verified zero live
  consumers/occurrences) filed as RETIRE-DEAD-DECLARED-SURFACE; the
  `sdk/src/assembly.ts`/`emit.ts` residue is evidence only, correctly
  no-entry (dies with the already-OPEN `(manifest-authoring-surface)` fork's
  0015/0016 derivations) — left unfiled per the report. The `import`-verb CI
  break the report bundled into "no entry" was independently re-scoped: it
  has no dependency on 0015/0016, `import` is confirmed absent from
  `src/main.rs`'s `Command` enum, and CI is a hard gate — filed
  CI-DOCUMENTED-TWO-LINE-JOB rather than deferring it. All three new entries
  are disjoint from each other and from PACKAGING-CHANNELS (unchanged,
  carried forward — its gate reason is job 3's to re-verify, not touched).
- Queue: 4 — CI-DOCUMENTED-TWO-LINE-JOB, SDK-VERSION-LOCKSTEP,
  RETIRE-DEAD-DECLARED-SURFACE (all open, pickable), PACKAGING-CHANNELS
  (still parked). Disjoint file sets, verified.

Plan continues: yes — spec delta is live (469e508 "install installs the tool,
whole" / 0016, e48e33b "no middles" / 0017, past cursor 474835b) and was not
this tick's job; the next tick takes job 2.
