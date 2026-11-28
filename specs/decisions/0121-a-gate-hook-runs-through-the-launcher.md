# 0121 — a gate hook runs through the launcher

- **Date:** 2026-11-27 · **Status:** accepted

## Context

Plan asked which path a package-bin gate tool runs (question
gate-tool-default-path-to-the-engine). Through `.bin/temper`, a Node launcher,
a guarded PreToolUse Edit takes 39 ms at p50, against 7 ms for the engine
binary called directly (0.0.26, this repo). `distribution.md` item 2 said no
placement needs a language runtime, which the launcher seemed to contradict.

## Decision

- **The gate hook keeps the anchored `.bin` launcher.** Item 2's property is
  that no verdict comes from running the program, and that the engine needs
  no runtime of its own. A package-installed gate may reach the engine
  through Node.
- **A gate that cannot reach the engine exits 2** (same day, `distribution.md`
  fail-loud invariant), whichever path it runs.

## Rejected

- **The hook names the platform engine's own path.**
  - Committed settings are shared across platforms, and the platform
    package differs on each one.
  - Whether a transitive dependency's bin is linked depends on the
    consumer's package manager and its settings, so the gate cannot name
    one. Probed 2026-11-27 with `make-dir@3.1.0` as the only dependency:
    pnpm 10.26.2 at its default isolated layout links none, while
    `node-linker=hoisted`, `shamefully-hoist=true`, or npm 11.6.2 links
    `semver`'s bin.
  - A missing path exits 127, which Claude Code treats as non-blocking.
  - The 30 ms saved is well under 1% of a turn dominated by model time.
- **A shell wrapper that probes for the binary and falls back to the
  launcher.** Reopen it if field timings show the launcher's start
  matters. It needs the Windows hook shell probed first.
