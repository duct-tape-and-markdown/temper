# 0083 — the closure is what the repository tracks

- **Date:** 2026-11-24 · **Status:** accepted

## Context

0080 fingerprints "every local module the run loaded, `node_modules`
excluded". The shipped filter compares real paths. This repo's SDK is
linked by `file:`, so its real path falls outside `node_modules`, and a
re-emit produced 14 rows for `sdk/dist/**`. That directory is gitignored
build output: a fresh clone reads the root as stale until the SDK is
built, and every SDK rebuild reads as drift. Any adopter with a
workspace-linked or `file:` SDK hits the same rows. The question
`module-closure-dependency-boundary` proposed treating anything reached
by package name as a dependency. Cascade's protected list, GH #61's own
case, is reached that way (`@cascade/tools/protected`), so that rule
brings back the false green 0080 exists to remove. Ruled by John
2026-11-24 on the session's recommendation.

## Decision

**The fingerprinted closure is the modules the run loaded that the
repository tracks** (`authoring.md`, "The SDK"). The reasoning starts from
what freshness asks: would emit still produce the committed bytes? Emit's
output depends on three inputs, and each already has its own guard:

- **Authored modules** are fingerprinted.
- **The toolchain** is pinned by its version, which the lock names
  (`engine-matches`).
- **Third-party packages** are pinned by their lockfile.

A gitignored file is not authored, so it is not fingerprinted. CI's
re-emit and byte-compare stays the exact check behind all three.

## Rejected

- **Package resolution as the boundary.** It drops #61's workspace
  package.
- **Stopping at the harness root.** It drops a monorepo's committed
  helpers above `.temper/`.
- **No import fingerprinting at all,** leaning on CI's re-emit. The
  session-start report would again call a stale harness fresh, which is
  the field report.

## Consequences

The closure filter consults the repository's ignore rules, which
discovery already honours, by real path. The `module-closure-dependency-
boundary` question closes. The dogfood lock is re-emitted once the entry
ships.
