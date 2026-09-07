## Symptom

`cargo test` (afterMerge gate) took **427s in `tests/hook_kind.rs` alone**, all
of it inside `two_kinds_at_the_same_collection_address_trip_collision_loud`
("has been running for over 60 seconds", then `ok`). `ps` during the stall
showed the test's spawned child — `target/debug/temper check --harness
/tmp/collection-address-collision<rand> --reporter github` — alive for 400s+.

Not local to this worktree: at the same moment a *sibling* fanout worktree
(`layout-prose-region-unreachable-lock-explain`) had its own identical
`hook_kind` child stalled on its own `/tmp/collection-address-collision<rand>`.
Two concurrent build ticks, same test, same stall — so it reproduces under
fanout concurrency, not from either tick's diff (neither touches the collision
path). Serially, after killing the stragglers, the suite completed exit 0.

## Cost this tick

~12 minutes: three `cargo test` invocations timed out at the tool's 120s/420s
ceilings and were backgrounded, one of which I then had to diagnose as *not*
my regression (killing stale children, re-running the suite alone) before I
could trust the gate. No revert — the test does eventually pass.

## Suggested fix

Diagnose why the collision fixture's spawned `temper check` blocks (an
inherited-stdin wait, or a pipe-full deadlock in the test's child-process
handling, would both explain "slow only when other processes contend"), then
either fix the spawn or bound it. Product work if the stall is in `check`
itself — route to `.flume/inbox.md`; test-harness work if it is the fixture's
`Command` plumbing. Either way a 7-minute single test inside the afterMerge
gate is a tax every fanout wave pays.

## Diagnosis (2026-09-07, `nested-member-duplicate-key-admissibility`)

The spawned `temper check` is not blocked — it is **CPU-bound walking all of
`/tmp`**. Sampling the hung child's open fds shows it descending
`/tmp/check-cost<rand>/packages/pkg-836/src`, `/tmp/check-cost<rand>/.claude/skills/skill-1048/scripts`,
`/tmp/admit-dangling-verifier<rand>` — every *other* suite's fixture tree, none of
them under the `--harness /tmp/collection-address-collision<rand>` it was given.
`/tmp` here: **9.0 GB, 37,222 entries**.

That explains the fanout correlation without any deadlock: `common::tmpdir`
persists every fixture with `.keep()` (documented — callers hand paths across
process boundaries), so `/tmp` grows monotonically with every suite run, and this
test's cost grows with it. Fanout just fills `/tmp` faster. It is not a hang; it
is an O(size of `/tmp`) walk. Left alone for 20+ minutes it was still making
progress, spawning fresh children each re-walking the whole directory.

Two fixes, independent: stop the walk escaping the harness root (a fixture rooted
directly under `/tmp` should not make `/tmp` the discovery root — that is product
behaviour worth checking, `.flume/inbox.md`), and give the fixtures a scoped
parent (`tempfile` under one per-run dir) so debris cannot accumulate at `/tmp`'s
top level.
