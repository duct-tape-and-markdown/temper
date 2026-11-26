# An edge's target set names kinds by import, never by string

`specs/model/representation.md`, "kind", and `specs/builtins.md`: kind identity travels by import, never by string. Edge target sets name kinds by string (`sdk/src/kind.ts:96` `to: readonly [string, ...string[]]`; built-ins `to: ["tool"]`, `to: ["output-style"]`, `to: ["known-marketplace"]`), and so does a requirement's `kind` (`sdk/src/contract.ts:530`). This is intent the code does not meet (0105). `to:` and the requirement's `kind` take kind values. The lock still records names, so the seam is unchanged. Acceptance:
- **new:** `to: [skill]` (a kind value) compiles and resolves;
- **new:** a misspelled kind fails `tsc` instead of failing at `check` (SDK type test);
- **kept:** the lock's edge rows are byte-identical.
