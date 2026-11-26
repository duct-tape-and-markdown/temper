# Tool lockfile read stops at the workspace root

The walk now reads the program's own directory, then stops at the nearest
ancestor declaring a workspace (`pnpm-workspace.yaml`, or `package.json`
with a `workspaces` array) and reads its lockfile only when that declaration
includes the program (positive glob matches, no `!` glob does). Undeclared
ancestors' lockfiles, yarn's included, are never read and never refuse.

Choices worth a look:
- A pnpm or npm declaration found in the same directory joins its globs, and
  any lockfile there is read. Neither manager is paired with its own lock.
- The object form `workspaces: { packages: [...] }` (yarn classic) is
  uncited and not read. A yarn workspace that uses it gets the advisory
  tool.unlocked finding instead of the yarn.lock refusal.
- A declaration that can't be read or parsed declares nothing. The run
  isn't refused over a broken ancestor manifest.
- Mixed nesting, where the nearest declaring ancestor excludes the program
  and a further one includes it, stops at the nearest. That is pnpm's
  find-up rule. npm's exact walk was not verified beyond its docs' "prefix
  is set to the root workspace".
- The SDK side uses node:path `matchesGlob`, which is experimental in
  Node 22. It ran with no warning on 22.21.
- The existing npm workspace-root test now writes the root `package.json`
  carrying `workspaces`, as a real npm workspace does.
