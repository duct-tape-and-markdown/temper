# Launcher exits 2 when the engine is unreachable

Shipped: `sdk/bin/temper.js` exits 2 on unsupported platform, missing
platform package, and spawn failure (spawn failure now also prints the
reinstall line). `sdk/test/launcher.test.ts` covers all three plus the
engine-status pass-through; the spawn/pass-through cases seat a shell-script
fake engine and skip off linux-x64. The unsupported-platform case spoofs
`process.platform` via a `--import` data-URL preload.

Open edge, not decided here: an engine killed by a signal (`status` null)
still exits 1 (`result.status ?? 1`), i.e. fails open under a gate hook.
Whether a signal-killed engine counts as "cannot reach the engine" is for
the spec to say.
