# Package-bin tools read the lockfile

Shipped: `src/tool.rs` reads `package-lock.json` (v1-v3, following a `file:` link) and
`pnpm-lock.yaml` (via `gray_matter`'s YAML engine; yaml-rust2 is reachable that way, so
no crate was added) from the program's workspace. `check` adds the advisory fixed check
`tool.unlocked` (also when no lockfile exists at all); yarn/bun/npm-shrinkwrap alone
refuse `check` and `explain` by name, and `emit` where a reference renders. The tool `TOOL_*` leaf consts moved from
drift.rs into tool.rs.

Debt worth a plan look:
- Two lockfile readers. The reference renders in the SDK at emit, so
  `sdk/src/lockfile.ts` re-reads the lockfile (pnpm via an indentation walk, not a YAML
  parser). The engine cannot feed the SDK a version over the one pipe. They agree by
  test only (tool_kind.rs drives both on each format). Options: have the engine render
  the tool topic's version, or have the reference say "see explain".
- The reference's kind topic now names members (the `tool` topic's `## Commands`),
  which breaks the "a kind topic names no member" property reference.ts stated.
- `explain root/tool/<key>` is parsed as a malformed leaf address, and a bare key does
  not resolve: root-hosted tools are not in explain's member index. The tests use
  `member:root/tool/tsc`, which goes through only because the `member:` prefix skips
  resolution. Fixing resolution is its own entry.
- A pnpm 10 lockfile with an env document first (`configDependencies`): gray_matter
  reads only the first document, so it reads as "not in the lockfile".
- Lockfiles are read only from the workspace dir. A workspace-root lockfile above it
  (npm/pnpm workspaces) is not found.
