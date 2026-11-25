# GATE-INSTALLED-JUDGES-THE-MEMBER

Shipped. `install.gate-installed` now judges a hook as a gate hook member when
the group at the gate address carries a command the engine renders for that verb
at *some* binary (`rendered_gate_binary`: unquote the probe word, re-render,
byte-compare). So an off-PATH binary reads installed. `group_has_command` is
gone. Scaffold writes `gateHook({ name, matcher })`. The stale remedy names that
constructor. The SDK no longer exports `GUARD_COMMAND`/`SESSION_START_COMMAND`,
and `generated/gate-commands.ts` is retired.

Two things for plan:

- **`GATE_COMMAND_BINDINGS` still stands in `src/install.rs`, with no reader.**
  `src/builtin_kind.rs:758` (the `TAP_HOOK_BINDINGS` doc) links it, `lib.rs`
  denies broken intra-doc links, and builtin_kind.rs was outside this fence. To
  retire it, a follow-up edits that doc line and deletes the const.
- **The judgment reads the projection, not the lock.** A lock registration row
  drops its `fields`, so nothing persisted says "this member is a gateHook".
  So a plain `hook(...)` whose command is a literal copy of the rendered bytes
  reads as installed, because the two are byte-identical in settings.json. If
  "judges the member" has to mean the member's type itself, the lock row would
  need a gate fact (`drift.rs` + `RegistrationRow`). That is a seam change, so
  it needs a decision.

Wired now also requires the seated group's matcher to match (address plus gate
rendering). Before, any group under the event counted.
