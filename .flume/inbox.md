<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

## TAP-WORKTREE-GIT-AWARE's shipped fix mishandles relative commondir — defective against real git layouts — observed at b787061b

The 6e94a6c9 fix resolves the worktree chain, but real `git worktree add`
writes a **relative** path into `commondir` (`../..`, relative to the admin
dir — verified by local probe: `git init` + `git worktree add` + inspect;
gitrepository-layout(5) says relative-to-$GIT_DIR). `resolve_log_path` does
`PathBuf::from(commondir_content.trim())` with no join onto `admin_dir`
(src/tap.rs:183-191), so a relative commondir resolves against the **process
cwd**: from a hook cwd'd in a flume worktree, `../..` → parent chain of the
cwd, and records land at `.flume/jobs/<job>/worktrees/.temper/tap.jsonl` —
the loss relocated, not fixed. The shipped test passes only because it writes
an **absolute** path into its synthetic commondir (tests/tap.rs:236-241), a
layout real git never produces.

Second, latent: the `.git` file's `gitdir:` pointer can also legally be
relative (to the directory containing the `.git` file — newer git supports
relative worktree links); same missing-join shape at src/tap.rs:179.

Fix shape: join a relative commondir onto `admin_dir` before use, join a
relative gitdir onto the harness root, and normalize (the `..` segments must
collapse or `parent()` walks the wrong lexical chain). Re-cut the test's
synthetic layout to write `../..` the way git does — or build the fixture
with real `git worktree add` so the test can't drift from git's actual
layout again.

