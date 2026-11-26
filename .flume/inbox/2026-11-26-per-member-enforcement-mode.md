# The enforcement mode is overridable per member

`specs/model/representation.md`, "The root member": harness-wide declarations, the enforcement mode among them, are "overridable per member". The code holds one harness-wide mode (`sdk/src/declarations.ts:967` emits a single `mode` row, and `src/compose.rs:2598` reads it), and no member or kind carries a mode. This is intent the code does not meet (0105). A member, or a kind, declares its own mode, and the guard reads the most specific one. Acceptance:
- **new:** a member declaring `block` is blocked while the harness is in `warn`;
- **kept:** with no override, the harness mode governs.
