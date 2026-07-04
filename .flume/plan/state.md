# Plan state

- **Phase:** reconcile. HEAD 0dd318a.
- **Last shipped:** the SDK altitude wave's head landed — `build` 272b4f4
  (SDK-EMIT-BYTE-PARITY: `sdk/src/emit.ts` brought to byte-parity with the Rust
  `toml_edit` manifest output) + `chore(flume)` 0dd318a. Verified on disk:
  emit.ts carries the parity emitter; the two `bodyText` bounds (fromFile,
  mention) at emit.ts:369-383 still throw; no `project.ts`/`lock.ts` yet.
- **This tick:** dropped SDK-EMIT-BYTE-PARITY (shipped). Flipped
  **SDK-BODY-RESOLUTION** blockedBy→`open` (its upstream shipped);
  SDK-PROJECTION-LOCK stays `blockedBy SDK-BODY-RESOLUTION` (shared emit.ts,
  serialized). Drained the inbox: `(display-rule-emit-face)` RESOLVED (John,
  narrow slice) → filed pending **DISPLAY-RULE-GENRE** (`open`, src/ only) and
  marked the fork RESOLVED in open-questions. Deferred/parked entries unchanged.
- **Pickable now (parallel-safe, disjoint blast radii):**
  **DISPLAY-RULE-GENRE** (`src/display.rs` + `tests/display_rule.rs` + `lib.rs`
  `pub mod`) and **SDK-BODY-RESOLUTION** (`sdk/src/emit.ts`/`members.ts` +
  test) — src/ vs sdk/, no shared file. Nothing else: SDK-PROJECTION-LOCK
  blocked; EXTRACTION-VOCAB-GAPS / AGENT-KIND deferred (no consumer);
  PACKAGING-CHANNELS parked on human creds.
- **What's next:** build ships the two `open` entries in parallel; a following
  plan reconcile un-blocks SDK-PROJECTION-LOCK once SDK-BODY-RESOLUTION lands.
  Human forks still open: PACKAGING-CHANNELS creds, the genre-adoption pilot
  (staged human ceremony, now unblocked by DISPLAY-RULE-GENRE).

Plan continues: no — queue reconciled, inbox drained, two disjoint `open`
entries are pickable. Hand to build; building is how the queue drains.
