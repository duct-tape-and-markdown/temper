# HOOK-EVENT-NOT-NAME: hook()/gateHook() take `event`

Shipped. `HookInit` is now `Omit<MemberInit<Hook>, "name"> & { event }`, and
`GateHookInit.name` became `event`. Each keeps a `name?: Remedy<...>` slot, so
the old spelling fails `tsc` and the error names `event` (a refusals test pins
this). The install lift writes `event:` for a lifted hook module
(`identity_property` in src/install.rs) and `name:` for every other kind. The
gate-hook remedy call (`gate_hook_call`) now writes
`gateHook({ event, matcher })`, and the counsel line and gauntlet snapshots
follow it. The example harness is re-emitted: its temper-sdk.md projection
and lock moved.

Human halves still owed:
- `.temper/hooks.ts` (the dogfood) still calls `gateHook({ name: ... })`.
  That file is outside build's fence. It will fail the dogfood's typecheck
  until a chore(harness) commit renames the key and re-runs `temper emit`,
  which also re-projects the counsel-derived rule.
- Migration note line (docs/): `hook({ name })` / `gateHook({ name })` →
  `{ event }`. docs/MIGRATING-0.0.24.md still shows the old spelling, which
  is historical and correct for that version.
