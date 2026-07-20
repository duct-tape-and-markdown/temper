# Plan state

- Spec derived through: 946e303 — unchanged; `git log 946e303..HEAD -- specs/` is empty.
- Audited through: 32a4c9c — unchanged, no new src/tests/sdk commits since (`git log
  32a4c9c..HEAD -- src/ tests/ sdk/` empty).
- Residue swept through: 32a4c9c — unchanged, same reason.
- Posture swept through: sdk/src/declarations.ts (+ assembly.ts/kind.ts/contract.ts/prose.ts/
  builtins.ts) covered, plus sdk/src/claude-code.ts (covered earlier in this rotation) and now
  sdk/src/dial.ts covered too this tick — mid-rotation. sdk/src/emit.ts next in rotation (tree
  order; kind.ts/prose.ts already covered leaves emit.ts, index.ts, needs.ts remaining in
  sdk/src/, then the tests/ tree — phrase delta 2e2b32a still arms the whole sweep domain).
- This tick: POSTURE SWEEP, sdk/src/dial.ts neighborhood (immediate imports: contract.ts's
  clause/closedKeys/enumOf/required/type/Clause/Severity, kind.ts's kind/KindDefinition — both
  already covered from the declarations.ts sweep, re-read here for context only, not re-judged).
  Clean: single-purpose shipped-kind module, doc comment states and the code matches — dial is
  deliberately root-exported (not the claude-code provider subpath) since it is a claim about
  temper's own gate, not an external harness fact, so no embedded-provider-knowledge violation.
  `dial`/`dialDefaultContract`/`Dial`/`DialEntry` all have live consumers (index.ts root export,
  tests/builtin_lock_frozen.rs's real-SDK-emit seam gate, src/gate.rs's dial-skip logic) — no
  zero-consumer export. No cohesion or dead-plumbing finding. The one stale cite the neighborhood
  touches — contract.ts:338's `telemetry` doc comment still pointing at `TELEMETRY_EVENT_HOOKS` in
  declarations.ts though it moved to builtins.ts — is already the eleventh entry in
  open-questions.md's ride-only record; not refiled.
- Queue: 3 pending — 0 open, 1 deferred (GUIDANCE-FIELD-DECLARATION-CHANNEL), 2 parked
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks: 2 (multi-harness-projection,
  lazy-grounds), unchanged. Friction: 0. Amendments: 0. Inbox: 0 notes.

Plan continues: yes — the posture-sweep rotation remains open (sdk/src/emit.ts next) and is the
only live input; no pickable entry exists in the queue to interleave with it.
