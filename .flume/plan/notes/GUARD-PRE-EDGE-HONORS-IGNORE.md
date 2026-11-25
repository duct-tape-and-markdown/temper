# GUARD-PRE-EDGE-HONORS-IGNORE: pre edge asks discovery's single-path query

Shipped. `import::discovers_path(harness, relative)` is the single-path query: it puts
each prefix of the path through `fenced` (.git, workspace, nested governed root) and
the git ignore sources `ignore_walk` consults. It checks `.gitignore` deepest-first,
then `.git/info/exclude`, so an ignored directory prunes its children. A unit test in
import.rs pins it against the real walk on a fixture. `import::discovery_fence` is now
the one entry filter for both `discoverable_paths` and `install::locus_member_sites`.

The pre-edge call sits in `install::matches_governed_locus`, not in main.rs: `guard`
already has `root`, so main.rs needed no change.

Still open:
- `.`-rooted loci (memory's `**/CLAUDE.md`) still skip at the pre edge. Nothing proves
  the whole every-directory reach agrees with check, so this tick did not widen it.
- The query hand-mirrors the ignore crate's precedence, because the crate's
  per-directory `Ignore` stack is private. Its `saw_git` rule (parent gitignores
  dropped above a nested `.git`) is not mirrored. That is harmless with
  `require_git(false)` plus the `.git` fence, but a crate upgrade that changes the
  precedence would drift silently. The agreement test is the tripwire.
