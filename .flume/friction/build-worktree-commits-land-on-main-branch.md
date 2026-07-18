## Symptom
Building BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE, the build agent's `git
commit` from inside its isolated worktree (`flume/builtin-kind-definition-
result-collapse`) landed on `main`'s HEAD instead of the worktree's own
branch — three times in the same tick (reflog: d9ebcbd, 7f42ecd, 33ee095,
each immediately followed by a reset back to 7e082e9). The implementation
itself was correct and fully verified (cargo build/test/clippy/fmt all
green, per the bail record) — only the commit's target ref was wrong.

## Cost this tick
One full build tick voluntarily bailed after 3 failed commit+reset cycles:
~197 turns, ~795s, ~15.18M cache-read tokens (.flume/metrics.jsonl,
2026-07-18T13:56:33Z) — on top of an earlier same-tag failure before the
entry was split (181 turns/~19.5M cache-read tokens, 2026-07-18T12:14:48Z,
already attributed to oversizing and addressed by the prior split). This
second failure is a distinct cause: the bail record
(.flume/prior-attempts/builtin-kind-definition-result-collapse.json) names
it explicitly as a worktree/branch-routing bug, not a scope problem —
re-splitting the entry again would not fix it, so the entry re-queues at
its current (already-split, correctly-sized) scope for the next attempt.

## Suggested fix
Investigate why `git commit` executed from inside a worktree directory
resolved to `main`'s HEAD rather than the worktree's own branch — likely an
env/cwd mismatch between the Bash tool's working directory and git's
resolved `.git` (a stale `GIT_DIR`/`GIT_WORK_TREE` env var inherited from
the parent process, or the tool not actually `cd`-ing into the worktree
path before invoking git). A pre-commit sanity check in the build phase
(`git rev-parse --abbrev-ref HEAD` verified against the expected worktree
branch before every commit, refusing loudly on mismatch) would turn 3
silent wrong-branch commits + resets into one fast, clear failure instead
of burning a full tick.
