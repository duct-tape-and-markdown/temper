# `when()` makes its author state a severity, as `clause()` does

`specs/model/contract.md`, "clause": "The tool never decides error-versus-warning; the clause does." `clause()` requires a severity, but `when()` defaults one: `severity: opts?.severity ?? "required"` (`sdk/src/contract.ts:404`). So the tool picks error severity whenever a conditional clause's author leaves it out. Make `severity` required on `when()`'s options, as it is on `clause()`. The type then refuses the omission (0098 makes emit run the compiler), and the shipped `when` clauses state their severities explicitly. This is SDK-only, with no `tests[]` line. Acceptance:
- **new:** a `when()` with no severity fails `tsc` (SDK type test);
- **kept:** every shipped `when` clause still compiles with its stated severity.
