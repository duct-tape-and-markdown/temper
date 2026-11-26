# Emit typechecks the program before running it

`specs/model/pipeline.md`, "Emit": emit runs the declared typecheck "before running the program". Today it runs the program first (`src/drift.rs:1911`) and typechecks after (`:1918`), because the declaration lives in the payload. A mistyped program that throws at runtime then reports that error before tsc's diagnostics. This is intent the code does not meet (0105), and the EMIT-RUNS-THE-DECLARED-TYPECHECK build note raised it under "Open for plan". The declaration must be found before the program runs: statically, from a declared location the engine reads, or from the previous lock's typecheck row. Plan decides which. Acceptance:
- **new:** a program with a type error and a runtime throw reports the compiler's diagnostics, not the throw;
- **kept:** a well-typed program emits as today.
