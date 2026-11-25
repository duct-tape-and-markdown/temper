# DISCOVERS-PATH-HONORS-THE-WORKTREE-EXCLUDE: premise contradicted, walk does not follow `.git` files

The entry assumes `ignore_walk` resolves a `.git` pointer file (gitdir: then
commondir) before reading info/exclude. It does not, under the builder
`ignore_walk` configures. ignore 0.4.27 `dir.rs` `add_child_path`:
`check_vcs_dir = require_git && (git_ignore || git_exclude)`; `ignore_walk`
sets `require_git(false)`, so `git_type` is `None` and
`resolve_git_commondir` returns `dir/.git` unconditionally - the same
`dir/.git/info/exclude` path `discovers_path` (import.rs:750) joins.

Probed empirically (scratch test, removed): a root whose `.git` is
`gitdir: <common>/worktrees/linked` with `commondir = ../..` and
`<common>/info/exclude` naming `excluded.md` - the walk yields
`excluded.md`. So today both surfaces agree (discoverable to both);
building the entry as written would make `discovers_path` disagree with
the walk, contradicting the acceptance's own "as it is to ignore_walk".

The real gap is upstream of `discovers_path`: in a linked worktree (this
repo's flume worktrees included) `check`'s walk ignores the common
info/exclude. Fixing it is a design call, not a build fix:
- `require_git(true)` makes the crate resolve the pointer, but drops
  `.gitignore` honoring in non-git trees (sub-trees, fixtures) that
  `ignore_walk`'s doc says is deliberate;
- or hand-resolve the common dir and feed it via `add_ignore`, a second
  hand-roll of what the crate keeps private (`resolve_git_commondir` is
  not `pub`), plus the same in `discovers_path`.
Plan should decide whether worktree exclude is in contract
(specs/model/adoption.md) before re-entering this.
