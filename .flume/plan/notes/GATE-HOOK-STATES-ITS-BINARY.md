# GATE-HOOK-STATES-ITS-BINARY: shipped; the fact rides the registration row, not a new family

The entry predicted a new `Declarations` row family (re-blessed
`Declarations.ts`). The fence ruled that out: `tests/it/lock_declaration_rows.rs`
builds `Declarations` literally with no `..Default`, so any new field breaks it,
and the same goes for `RegistrationRow` (literals in four out-of-fence test
files). The binary instead rides the gate hook's registration row as one
seam-inbound field under the reserved key `temper:gate-binary` (`null` = none
declared). `drift::gate_rendered_fields` consumes the key and renders handler
0's `command` through `install::gate_hook_command`. The default `temper` is
spelled only in the engine (`DEFAULT_GATE_BINARY`).

Debt for GATE-INSTALLED-JUDGES-THE-MEMBER, which owns the seam gate:
- The key is spelled twice, in `drift.rs` `GATE_BINARY_KEY` and `builtins.ts`
  `GATE_BINARY_KEY`. Generate the SDK side across `generated/` from
  `tests/it/seam_bindings_current.rs`, or promote it to a typed row once that
  entry's fence can reach the literal constructions.
- The fact is not written to the lock: a lock round-trip reads it as absent.
  For check to judge the member it must either be written there or read off
  the projected command.
- `SESSION_START_COMMAND` and `GUARD_COMMAND` stay as consts, pinned equal to
  the renderer at `temper` by `tests/it/install.rs`. The scaffold still mints
  modules that import them, and the installed-gate check still compares
  strings, so a declared workspace binary still reports stale until that entry.
- The SDK side keeps the fact in a `WeakMap` keyed by member, so the member's
  typed fields stay exactly the manifest entry.
