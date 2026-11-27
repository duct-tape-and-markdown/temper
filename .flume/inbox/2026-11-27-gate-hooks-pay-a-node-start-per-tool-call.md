# Every gate hook pays a Node start before the engine runs

Found measuring the Rust-or-TS question (ledger). The default gate tool is the package's bin (`specs/model/authoring.md`, "a package's bin … the anchored `.bin`"). For `@dtmd/temper` that bin is `sdk/bin/temper.js`, which starts Node and then `spawnSync`s the platform engine. Each guarded tool call therefore pays Node's startup (about 22 ms on its own) on top of the engine. Measured with the published 0.0.26 package over this repository (p50 of 60 runs, 4-core container):

| payload | through `.bin/temper` | engine binary direct |
|---|---|---|
| PreToolUse Edit, unowned file | 39 ms | 7 ms |
| PreToolUse Edit, projected file | 40 ms | 7 ms |
| PostToolUse Bash | 48 ms | 18 ms |

The platform package (`@dtmd/temper-linux-x64`) declares no `bin`, so no anchored path reaches the engine without the launcher. Today's spec holds the shim as the default: the launcher's fail-loud install error is the reason it exists (`specs/distribution.md`, "What ships"). Moving the default needs a spec change before any entry. One option is a default gate tool that names the platform engine's own path; Node has no in-place exec, so the launcher alone cannot remove the hop.

Acceptance: whichever the spec settles, a missing platform binary still fails loud with the install instructions. The latency is a manual signal, never a test (engineering.md: a pin is a count, never a clock).
