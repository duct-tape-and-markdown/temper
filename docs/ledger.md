# Session ledger — cross-session parking lot

Maintained by the interactive session assistant; read on demand (session
open to pick the one focus, resuming parked work), never imported. One
rule: this is a board of pointers, not a narrative — design reasoning
lives in the records it links, work orders live in flume, ratified
decisions live in the corpus, session conduct lives in `.claude/rules/`,
and all of those are forgotten here once homed. Target under ~60 lines,
hard.

## State of the era (2026-07-16)

- **The center (0019)**: temper types the documents that program agents;
  the launch demo is this repo's spec corpus governing itself. Kernel
  corpus in `specs/model/`; decisions outside every read path.
- **META-FREEZE (John, 07-09)**: no harness/process/audit investment this
  side of v0.1. Amendments: manifest campaign admitted (0021, 07-10);
  harness-surface settle sanctioned 07-16 — flume's operating layer
  thinned to a directive, `.flume/PROTOCOL.md` its one home (c4a73ed).
- **Distribution**: channel 2 live 07-11 — `npx @dtmd/temper` delivers
  prebuilt linux/win32 engines (SDK 0.0.7, release.yml, NPM_TOKEN repo
  secret). Darwin + plugin channel ride PACKAGING-CHANNELS-REMAINDER
  (parked in pending.json); 0.1.0 is the tag's to stake.
- **Consumer campaign closed 07-16**: posture-recursion ruled — 0025
  (82c816e, amended cc5a9b33), prototype at
  `docs/proposals/posture-recursion/`; the built-in adoption is flume's
  as SKILL-NESTED-REFERENCE-DOCS. Open forks live in
  `.flume/plan/open-questions.md` (four; none block the queue).

## Parked (pointers only)

- flume 0.3.1 publish (John): three 07-18 runtime fixes are LIVE via a
  patched installed dist in temper's node_modules (ephemeral — a
  reinstall wipes it) and mirrored on flume branch
  fix/worktree-escape-and-loop-lock (written against 0.2.0 source;
  reconcile with wherever 0.3.0's source lives). Fix 1: worktrees
  relocate outside the repo (FLUME_WORKTREES_DIR; temper's chain sets
  ~/.cache/flume-worktrees/<repo>) — root cause of the stray writes
  was models deriving the root checkout from the worktree path prefix.
  Fix 2: loop pidfile lock refuses a second supervisor. Fix 3: a
  merge-reverted entry's actual commit footprint persists as
  entry.observedFiles and joins the partition, so retries never ride
  with what they collided with. The wave-chaining auto-unblock rides
  the same branch.

- Guidance layer: 4 source-verified deltas awaiting curation —
  claude.ai/code/artifact/97362c3b-f2eb-4e2a-98de-7a19a29855c8.
- Verify queue: trailing-period @import (cascade CLAUDE.md:26, UNVERIFIED).
- Docs-language candidate (post-freeze): the determinism ladder — "push
  every check to the most deterministic layer that can express it".
- Base harness dogfood: primer `docs/base-harness-primer.md`; example at
  `examples/base-harness/` (third cut shipped 549969f); built-in-kind doc
  audit at `docs/market-formats.md`. Sequencing: stranger dry run next,
  then channel 3.
- On John:
  Apple Developer notarizing (decide at release); USPTO name screen;
  CHANGELOG for the shipped 0.0.x npm cuts.

## Standing discipline (mechanical, paid for)

- Wake-then-loop as its own background task; `git status` before any
  restore; never edit tracked files while a tick runs (plan stages -A).
- At session open: sweep `.flume/friction/` and `.flume/refactor/`;
  delete `.flume/prior-attempts/` records whose entry re-scoped or
  shipped (write-only to plan — `.flume/PROTOCOL.md` has the rule);
  prune `.flume/sessions/` when it swells.
- Per green tick: verify commit, fence check (`git show <sha> --name-only
  --format= | grep -cE '^(\.claude|docs|specs)/'` = 0), push to origin.
- `cargo install --path .` after engine waves; `cargo insta test --accept`
  for snapshot churn.

## Goal: v0.1 release (set 2026-07-03; repo PUBLIC 2026-07-05)

Launch gate per `specs/distribution.md`: prebuilt binaries on three OSes,
stranger-proof quickstart, regenerable demo, USPTO screen on John. Weigh
every new thread against shipping this — the meta-freeze holds until the
tag.
