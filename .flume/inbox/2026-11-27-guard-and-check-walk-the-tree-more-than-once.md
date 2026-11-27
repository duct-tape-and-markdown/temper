# `guard` walks the tree six times per shell-edge call, and `check` three

Found measuring the Rust-or-TS question (ledger). The published 0.0.26 binary was run over a generated fixture: this repo plus N one-line files under `vendor/`, and N/2 more under a gitignored `target/`. Walks were counted with `strace` as opens of one fixed directory per run. `guard .` on a `PostToolUse` Bash payload opens `vendor/d0` 6 times, and `check .` opens it 3 times. `specs/process/engineering.md` ("Cost scale is hoisted, and pinned by count") states one walk per run.

The cost is in the engine's own code, not the filesystem. At 100k files, `check` spends 2.2 s of user CPU against 0.1 s of sys time, and the shell-edge guard spends 0.71 s user against 0.14 s sys. Wall time (p50, 4-core container):

| fixture | guard, shell edge | check |
|---|---|---|
| 10k files | 111 ms | 165 ms |
| 100k files | 912 ms | 2.4 s |

A naive single-walk TypeScript prototype of the shell edge ran 276 ms on Node over the 100k tree. `check` runs at SessionStart, so an adopter with a large tree waits on it before the session opens.

Acceptance:
- **new:** a count pin: one tree walk per `check` run and one per shell-edge `guard` call over a generated fixture;
- **kept:** both verbs report the same findings over this repository as they do today.
