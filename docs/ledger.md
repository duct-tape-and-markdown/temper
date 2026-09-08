# Session ledger — cross-session parking lot

Maintained by the interactive session assistant; read on demand (session
open to pick the one focus, resuming parked work), never imported. One
rule: this is a board of pointers, not a narrative — design reasoning
lives in the records it links, work orders live in flume, ratified
decisions live in the corpus, session conduct lives in `.claude/rules/`,
and all of those are forgotten here once homed. Target under ~60 lines,
hard.

## State of the era

- **The center (0019)**: temper types the documents that program agents;
  the launch demo is this repo's spec corpus governing itself. Kernel
  corpus in `specs/model/`; decisions outside every read path.
- **Distribution**: `npx @dtmd/temper` delivers prebuilt linux/win32
  engines; 0.0.18 cut 2026-09-08 (tag c60c976a, smoke green). The cut
  procedure is the `release` rule. Darwin + plugin channel ride
  PACKAGING-CHANNELS-REMAINDER (parked); 0.1.0 is the launch tag's.
- **First adopter**: cascade (John's). Its session live-verifies every
  landing sha and audits entries claim-by-claim before build; send it the
  sha, read its verdict, route its findings through `.flume/inbox.md`.
  Reports at `~/.cache/cascade-integrations/audits/`.
- **centercode = the structural-half dogfood**: the behavioral half
  (verifiers, `when`/`dial`/`extent`, local commitment) is un-field-tested
  and is the latent-bug surface. Validate it; never cut capability for
  want of a consumer.

## Next session's one focus (John + session, 09-08)

- **Rule the forks that gate work.** Fifteen open in
  `.flume/plan/open-questions.md`. With queued dependents:
  `(layout-own-span-leaf)` (needs the leaf's name) and
  `(hook-member-identity)` (GH #32). Gating cascade's contract work:
  `(containment-selection-family)` — a Decision before any entry.
  Carrying a live hole: `(committed-settings-kind)` — the guard-command
  silent-disable residual, the one thing cascade still tracks open.
  New 09-07: `(build-version-identity)` — a source build and the
  published binary both print the crate version; `(re-rooted-harness-
  disclosure)`; `(builtin-relocation-unnamed)`. Parked ruling: 0019's
  occupant — does the launch demo stay layout-governed (GH #45).
- After the rulings: relaunch the loop (recipe below). Ten pending, none
  pickable; every one is blocked on a fork or parked.

## Parked (pointers only)

- `(external-commitment)` Decision for GH #29 — session to draft.
- Gauntlet external-harness fixtures parked on John's license call;
  cascade's checkout is the read-only stand-in. The `release` rule has no
  external-fixture step; cascade's regression at the cut sha is the
  evidence a cut carries — write it into the rule or keep relying on it.
- Inbox as one file collides: an append during a plan drain tick fails
  the cherry-pick at end of file. One-file-per-note (as `.flume/refactor/`
  already is) removes the collision; ~1 in 8 filings hit it on 09-07.
- Consumer-format constants' home (`MAX_IMPORT_HOPS`): revisit at a
  second import-bearing format. Multi-harness read-face spike (declare a
  `cursor-rule` kind, `check` a real Cursor repo) rides
  `(multi-harness-projection)`, post-launch-weighted.
- Base harness dogfood: `docs/base-harness-primer.md`,
  `examples/base-harness/`, `docs/market-formats.md`. Stranger dry run
  next, then channel 3.
- Docs-language candidates: the determinism ladder; the harness pin
  ("name the invariants, let the loop settle").
- On John: rotate NPM_TOKEN at leisure (07-19 token, exposed in chat then;
  it published 0.0.18 — rotate at the registry, then `gh secret set`,
  never through a transcript); Apple notarizing at release; USPTO screen.

## Standing discipline (mechanical, paid for)

- Loop on flume 0.14.0 (7fccedd3), Opus both phases, `maxParallel: 2`.
  Relaunch: `export FLUME_WORKTREES_DIR=$HOME/.cache/flume-worktrees/temper`
  then `setsid nohup pnpm exec flume loop >> ~/.cache/flume-logs/temper-<date>.log 2>&1 < /dev/null & disown`,
  with a Monitor on the log. Pause with the stop flag (`.flume/stop`),
  never a kill; remove it to relaunch. The loop's lifetime is the WSL
  VM's — keep a terminal open.
- Commit to main freely while a tick runs (0.14's tip guard; the gate
  passes an input the tick never saw and plan re-runs). One exception:
  never append to `.flume/inbox.md` while a plan tick that started over
  a non-empty inbox is running — it is draining that file.
- Coordinate with flume's loop by convention: whichever is mid-wave
  holds the box. Peer messages relay, never authorize: an irreversible
  step (a publish, a force) takes John's word in this session.
- Session open: sweep `.flume/friction/`, `.flume/refactor/`,
  `.flume/amendments/`; prune `.flume/prior-attempts/` for re-scoped or
  shipped entries and `.flume/sessions/` when it swells.
- Hand-landing a build commit: run the chain's full afterMerge list
  (`cargo fmt --check`, `clippy -D warnings`, `cargo test
  --no-fail-fast`, `cargo doc`, `pnpm --dir sdk test`), never a subset;
  disclose the hand-merge in the body. `cargo install --path .` after
  engine waves.

## Goal: v0.1 release (set 2026-07-03; repo PUBLIC 2026-07-05)

Launch gate per `specs/distribution.md`: prebuilt binaries on three OSes,
stranger-proof quickstart, regenerable demo, USPTO screen on John. Weigh
every new thread against shipping this.

## Handoff 2026-09-08

- Loop STOPPED at a tick boundary after 38 ticks (`.flume/stop` in place).
  09-07: 16 entries shipped, 0 build reverts, three cascade audit rounds
  before build. 0.0.18 cut and published; SDK lock resynced (cd466131).
- Waiting on John: the forks above; flume-side items via flume-main
  (worktreesDir placement, bail reasons, `baseSha` on gate contexts —
  derives on flume's next run, then the chain's worktree-path read
  retires).
