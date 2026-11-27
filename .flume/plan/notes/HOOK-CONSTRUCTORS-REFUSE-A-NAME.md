# HOOK-CONSTRUCTORS-REFUSE-A-NAME shipped

The Remedy strings for `hook`/`gateHook` `name` now live once as consts in
`sdk/src/builtins.ts` (`HOOK_NAME_REMEDY`, `GATE_HOOK_NAME_REMEDY`), typed into
the `Remedy<typeof ...>` slots and thrown by `refuseAuthoredName`, so the tsc
and runtime refusals cannot drift apart.

Scope held to the entry: only `name` without `event` refuses. The other moved
fields on `HookInit` (`type`, `command`, `timeout`) and `gateHook`'s `binary`
still pass silently at runtime in an untypechecked program, and an init with
neither `name` nor `event` still builds a member named `undefined`. Same class
as the inbox record; plan may want one entry generalising the Remedy-slot
runtime refusal across every SDK init.
