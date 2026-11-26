# Nondeterminism in the authoring program is a loud emit failure

`specs/model/pipeline.md`, "Byte-reproducible": nondeterminism in authoring code is "a loud emit failure, never silent churn". The double-emit check (`src/drift.rs:4237-4247`) renders one parsed payload twice, so it tests only the engine. A `Date.now()` in the program is caught only later, by CI's `emit --frozen`. This is intent the code does not meet (0105). Emit detects a nondeterministic program itself, for example by running it twice and comparing payloads, weighed against the extra Node run. The cost is measured first (`engineering.md`, cost). Acceptance:
- **new:** a program that interpolates `Date.now()` refuses at emit with `DriftError::Nondeterministic`;
- **kept:** a deterministic program emits as today.
