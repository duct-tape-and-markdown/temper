# Plan state

- **Phase:** reconcile. HEAD e7fd54f.
- **Last shipped:** the two floor-follow-on fixes drained — RUNG-DECODIFY-CODE
  (`build` f4af8f0) and INSTALL-NOTE-CONTENT-DRIFT (`build` cae8e5a), shipped as
  `chore` e7fd54f. `rg -i rung src/ tests/` → empty; install's managed-by note is
  now content-drift-aware (re-places on a body change, not just on absence).
- **This tick:** reconciled all six remaining entries against the corpus — no
  `specs/` commit since d4a6999 (the rung de-codify, already shipped), so the code
  is reconciled to the scripted-altitude corpus and there is **no new gap** to
  file. Inbox empty. Verified on disk: `Primitive::Field` still flat-reads
  (EXTRACTION-VOCAB-GAPS deferred holds), `BUILTIN_KINDS = ["skill","rule"]` with
  no agent kind (AGENT-KIND deferred holds), the SDK scaffold is present, and
  `(sdk-build-gate)` is still unresolved (no `sdk` test gate in `.flume/chain.ts`
  — only the cargo fmt/clippy/test + selfCheck gates), so the SDK wave stays
  parked.
- **Pickable now:** none. The queue is entirely human-gated — the altitude wave
  (SDK-EMIT-BYTE-PARITY → SDK-BODY-RESOLUTION → SDK-PROJECTION-LOCK) parks on the
  `(sdk-build-gate)` human `chain.ts` edit; PACKAGING-CHANNELS parks on human
  release creds; EXTRACTION-VOCAB-GAPS and AGENT-KIND stay deferred (no consumer).
- **What's next:** a human unblocks — wire a `sdk` `shellGate` into `chain.ts`
  (un-parks the SDK wave), set packaging creds, or scope the display rule
  (`(display-rule-emit-face)`). Accepted debt unchanged: temper's own
  `temper.toml`+lock predate MANIFEST-EMIT (a human `chore(harness)` `emit`
  regen); the dogfood self-gate stays unwired until the wave-end confirmation pass.

Plan continues: no — queue reconciled with no change to pending, inbox drained,
and no `open` entry exists (every entry is deferred or parked on a human fork).
There is nothing for build to pick; the queue waits on human action, not a plan
tick.
