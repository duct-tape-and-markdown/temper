# Two telemetry declarations name one event with different binaries: one tap or two?

Driver: 0088 (`specs/model/pipeline.md`, "Telemetry") makes the binary each telemetry declaration's own fact, but tap hooks dedupe per (event, matcher) across declarations (`sdk/src/declarations.ts`, `tapHookRows`). Two answers fit every source: refuse the disagreement at emit, or synthesize one tap per (event, matcher, binary) — the second double-records each event. 0088's Consequences assume one shared constant and say nothing of disagreement. `TAP-HOOK-STATES-ITS-BINARY` ships the agreeing case and refuses a disagreement loud until this is ruled.

As shipped, the refusal compares raw facts, so an undeclared binary beside an explicit `"temper"` refuses as a disagreement. Defaulting in the SDK would spell the default twice (`specs/model/authoring.md`, "One fact, one spelling"). The ruling should say whether those two agree.
