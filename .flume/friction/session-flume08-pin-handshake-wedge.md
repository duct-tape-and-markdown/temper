## Symptom
flume 0.8.0's engine↔pin handshake (`src/cli.ts` `engineHandshake`, v0.7
§10) hard-wedges every bay whose **repo-root** `package.json` pins
`@dtmd/flume` — refusal (exit 2) in *every* provisioning configuration, not
just the unprovisioned one the migration guide names. Empirical matrix
(scratch bay, npm-published 0.8.0): (a) pin + no `.flume` install → refuse
(documented); (b) pin + `.flume/node_modules/@dtmd/flume` symlinked to the
bay's own install, invoked via that install → `readLocalInstall` sees the
link realpath == `OWN_PACKAGE_ROOT`, returns "does not resolve", falls to
the pin arm → refuse; (c) same layout invoked via a *different* engine →
arm 1 re-execs the link target, and the re-exec'd child then hits (b) by
construction — the final authority always sees itself in the link → refuse.
`flume job new` provisioning has the same terminal shape: the link target
is the running engine, so some hop always self-detects. The guide's "escape
hatch" (drop the pin) is in fact the only path; "provision the pinned
install" can never satisfy the handshake while `readPin` reads the repo
root. Root cause: self-reference detection ("I *am* the local install")
conflates with "no install resolves" and falls through to the refusal arm
instead of proceeding as the authority.

## Cost this tick
The 0.6→0.8 migration session burned the better part of an hour isolating
this: three scratch-bay experiments to prove no pinned layout survives,
then a layout migration the guide never mentions — the harness manifest
(package.json, lockfile, tsconfig, workspace anchor) moved from the repo
root into `.flume/` so the install resolves at `<flumeDir>/node_modules`
while `readPin(repoRoot)` finds nothing. Every 0.6 bay pinned at the repo
root (the layout flume's own docs used to suggest) hits this on upgrade.

## Suggested fix
Route to the flume repo's inbox: `engineHandshake` should treat
self-reference as "this invocation IS the provisioned install — proceed"
(a distinct `readLocalInstall` outcome, not a fall-through to the pin-arm
refusal), and MIGRATING-0.8 §4 should either document the flumeDir-scoped
manifest layout or `readPin` should also read `<flumeDir>/package.json`.
