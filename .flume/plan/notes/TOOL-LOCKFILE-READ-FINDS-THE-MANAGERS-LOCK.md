# The lockfile read finds the manager's lock

Shipped in `src/tool.rs` and `sdk/src/lockfile.ts`, which still agree only by test.
- The read walks from the program's workspace up to the nearest directory holding any
  lockfile. That directory decides: a read lock wins, and an unread one there alone
  refuses. The program's path below it is the importer. pnpm reads `importers[<path>]`.
  npm reads `<path>/node_modules/<name>` first, then the hoisted `node_modules/<name>`.
- pnpm locks are split at their bare `---` lines, and every document is read, because
  gray_matter's engine yields only the first. The two-document (boot/env) format is
  cited to https://pnpm.io/lockfile (retrieved 2026-11-25). That page does not say
  which pnpm version introduced the format, so the old note's "pnpm 10" was not
  encoded.

Observed:
- The walk is bounded only by the filesystem root. A stray lockfile in an unrelated
  ancestor (for example `~`) is read: an npm one draws `tool.unlocked` on a package it
  doesn't record, and a lone `yarn.lock` there refuses the run. Confirming the
  ancestor's workspace membership (`pnpm-workspace.yaml` / `workspaces`), or stopping at
  the repository root, needs a ruling.
- An unlocked finding still says "the program's `pnpm-lock.yaml`" when the lock is in
  an ancestor directory. It names the file but not where it is.
- npm v1 locks are still read from their top-level `dependencies` whatever the
  importer, because v1 predates workspaces.
- The `.bin` path a package tool runs at is still the program's own
  `node_modules/.bin`. In an npm workspace, the bin may be hoisted to the root's.
