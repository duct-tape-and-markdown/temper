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

## Next session's one focus (09-22 close)

- **Loop left running overnight** (flume 0.17, `CARGO_BUILD_JOBS=4`, VM 11 GB
  via `.wslconfig`; flume's own loop capped at vitest 4 workers / width 2 after
  two OOM reboots 09-22). Check `pnpm exec flume status` and the log first.
- **0.0.19: cut once** (John, 09-22) when SDK-MEMBER-DECLARES-INPUTS,
  SDK-REACHED-FROM-CONSTRUCTOR and LOCUS-DECLARED-CLAUSE-JUDGES-UNDECLARED
  have shipped; at close, ROOT-DEFAULT-CONTRACT-SHIPS was the next rung (3
  more under it). The publish takes John's word at the cut (`release` rule).
  Before it: check the consumer's five limits against disk, and run cascade's
  regression at the cut sha. Upgrade notes owed:
  - `settings.json` hand edits now drift (0050), and the guard no longer
    admits residue edits;
  - drift rule ids become clause labels (0054);
  - `when` labels carry the guard's values, and filtered `degree` labels carry
    their field set (0049 fixes);
  - set predicates in a `when` body are refused, not silently skipped. The
    platform consumer's guarded `degree`/`membership` hit this, so tell them
    to use kind partition;
  - `reachable` reports again once the root default ships;
  - settings residue is `residue: {…}`.
- **Waiting on John:** `(guard-body-address-and-severity)` in open-questions.
  Session recommends (a): a `when`-body clause gets its own label (owner =
  host label) and keeps its own severity. "Share one address binding" means the
  element binding, not the label.
- After the cut, unpark INTEGRATION-SUITE-ONE-TEST-TARGET before the harness
  migration (the judge resolves test ids the fold renames).
- Harness-migration carry list (re-verify in the package, or file with flume):
  - the ripple walk covers `examples/` and names a common type's exhaustive
    Rust literals and full TS spellings (00b1d690, 2ac1d6c7);
  - a blocked entry may edit a file its parent creates (59857dd5);
  - every cargo/nextest call is capped (jobs, test threads).
  The runner spike's verdict script was lost with /tmp. Its final text is in
  the 09-22 session transcript, and it lands in the repo at migration, along
  with the open 979-vs-~1,020 nextest gap.
- Still open with dependents: `(hook-member-identity)` (GH #32). Parked
  ruling: 0019's occupant (GH #45).

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

- Loop on flume 0.17.0 (09-22), Opus both phases, `maxParallel: 2`;
  worktrees off-repo via the chain's `worktreesBase`. Relaunch:
  `CARGO_BUILD_JOBS=4 setsid nohup pnpm exec flume loop >> ~/.cache/flume-logs/temper-<date>.log 2>&1 < /dev/null & disown`
  (the cap is load-bearing: default cargo on 20 cores linking ~60 test
  binaries, beside flume's own loop, OOM-rebooted the 7.7 GB VM twice 09-22),
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
- flume 0.17 shipped worktree placement, bail messages, and `baseSha`;
  the chain's plan-worktree reads (`PLAN_WORKTREE`) can move to
  `ctx.baseSha` — not yet taken.
