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

- flume runtime bug (John, `@dtmd/flume`): a build worktree's `git commit`
  resolved to `main`'s HEAD 3× in one tick (friction capture drained
  07-18, full text in git history; likely stale GIT_DIR/GIT_WORK_TREE or
  tool-cwd mismatch). Prompt-side stopgap shipped in build.md; the
  runtime fix is upstream.
- LSP-ahead-of-grep rule (parked 07-18, half-unlocked): root cause of
  the hangs found and fixed — rust-analyzer was rustup's proxy shim
  with the component never installed (exits instantly; the LSP tool
  layer waits forever on it — the missing-timeout half is upstream
  Claude Code's). `rustup component add rust-analyzer` fixed it;
  references now answer live and precisely. Remaining before encoding:
  verify inside a headless tick + per-worktree cold-index cost vs the
  cargo-gate contention trap.
- flume runtime gap (John, `@dtmd/flume`): no cross-process loop lock —
  two `flume loop` supervisors ran ~1h against one state root (07-18,
  operator error; history stayed linear on timing luck). A pidfile/lock
  refusing the second loop is the fix; session-side, launches now
  pgrep-check first.

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
