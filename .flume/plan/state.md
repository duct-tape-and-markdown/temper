# Plan state

- **Phase:** reconcile. HEAD 89e4d22.
- **Last shipped:** the SDK altitude wave's body-resolution link + the display
  rule landed — `build` 9641d9d (SDK-BODY-RESOLUTION: `sdk/src/emit.ts`
  `resolveBody` reads `fromFile` assets in and resolution-checks mentions
  against the declared address set, throwing loud on a dangling mention) and
  `build` 49a1a7d (DISPLAY-RULE-GENRE: `src/display.rs` `render_genre`, the
  per-genre formatter island). Verified on disk: emit.ts's former bodyText
  bounds now resolve (the remaining throws are byte-parity field guards, not
  scaffold bounds); `src/display.rs` + `lib.rs pub mod display` present; no
  `sdk/src/project.ts`/`lock.ts` yet.
- **This tick:** dropped SDK-BODY-RESOLUTION and DISPLAY-RULE-GENRE (both
  shipped, removed by `chore(flume)` 89e4d22). Flipped **SDK-PROJECTION-LOCK**
  blockedBy→`open` (its upstream SDK-BODY-RESOLUTION landed) and tidied its
  acceptance/notes to drop the blocked language. Inbox empty — nothing to
  drain. Deferred/parked entries unchanged.
- **Pickable now:** **SDK-PROJECTION-LOCK** (`sdk/src/project.ts` +
  `sdk/src/lock.ts` new, `sdk/src/index.ts` + `sdk/test/emit.test.ts` edit) —
  the wave's last link. Nothing else: EXTRACTION-VOCAB-GAPS / AGENT-KIND
  deferred (no consumer); PACKAGING-CHANNELS parked on human creds.
- **What's next:** build ships SDK-PROJECTION-LOCK, closing the altitude
  authoring wave (manifest + projection + lock in one deterministic SDK
  `emit`). Human forks still open: PACKAGING-CHANNELS creds, the
  genre-adoption pilot (staged human ceremony, unblocked by DISPLAY-RULE-GENRE).

Plan continues: no — queue reconciled, inbox empty, one disjoint `open` entry
is pickable. Hand to build; building is how the queue drains.
