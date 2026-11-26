# How does emit find the declared typecheck before it runs the program?

`specs/model/pipeline.md`, "Emit": "Before running the program, emit runs the typecheck the program declares." The declaration lives in the program's payload, so today emit runs the program first (`src/drift.rs:1911`) and typechecks after (`:1918`); a mistyped program that throws reports the throw, not tsc's diagnostics. Every reading the corpus admits collides with a standing claim:
- the previous lock's typecheck row: absent on a first emit, stale when the declaration changes, and it makes the lock an input;
- the engine reading the declaration statically out of TypeScript: the engine "sees only plain data" (pipeline.md, "Byte-reproducible");
- a declared location outside the program (a `package.json` field, a fixed tool name): a second spelling of one fact (`authoring.md`, "One fact, one spelling") unless the program reads it from there.

Recommendation: a fixed declared location that the program's `harness()` reads (one spelling, readable before the run). That needs a corpus ruling on where it lives. Inbox 2026-11-26-typecheck-before-the-program-runs and the EMIT-RUNS-THE-DECLARED-TYPECHECK note both raise it.
