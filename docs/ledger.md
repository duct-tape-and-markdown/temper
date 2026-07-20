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
- **META-FREEZE struck (John, 07-18)**: the 07-09 freeze no longer
  described a week of sanctioned harness work; its point — v0.1 ships
  before gold-plating — lives in the goal section. Loop may propose
  its own harness diffs via `.flume/amendments/` (0044,
  propose-and-ratify only); session-open sweep now covers it.
- **Distribution**: channel 2 live 07-11 — `npx @dtmd/temper` delivers
  prebuilt linux/win32 engines (SDK 0.0.10 cut 07-19; 0.0.8-0.0.9 deprecated — mismatched pins, enablement wire split; post-publish smoke gates every cut; release.yml,
  NPM_TOKEN repo secret). Darwin + plugin channel ride PACKAGING-CHANNELS-REMAINDER
  (parked in pending.json); 0.1.0 is the tag's to stake.
- **Consumer campaign closed 07-16**: posture-recursion ruled — 0025
  (82c816e, amended cc5a9b33), prototype at
  `docs/proposals/posture-recursion/`; the built-in adoption is flume's
  as SKILL-NESTED-REFERENCE-DOCS. Open forks live in
  `.flume/plan/open-questions.md` (four; none block the queue).
- **centercode = the structural-half dogfood (07-20)**: the consumer
  testbed exercises layout/requirements/graph/degree/count/nested-docs
  hard (9 embedded kinds, one factory); the behavioral half — verifiers,
  `when`/`dial`/`extent`, local commitment — is un-field-tested and is
  the latent-bug surface (`read_dial` + `when`-guard both lived there).
  Standing direction: validate it (dogfood + adversarial passes), never
  cut valuable capability for want of a week-old consumer; lean cuts fat
  (restatement, ceremony, over-claim), not capability-ahead-of-adoption.

## Next session's one focus (John, 07-18)

- **Govern `.flume/` with temper's own dogfood.** The authoring surface
  expanded (amendments channel, protocol slits, sweep frontier spanning
  prompt + rule + README, fence globs) and every shift now fans out
  across surfaces by hand narration — the drift class the product
  gates. Bring the flume operating layer under `.temper/` as governed
  members: prompts, PROTOCOL, capture READMEs as documents with layout
  contracts; each cross-surface fact declared once and projected.
  Open question `(.flume/ is ungoverned)` re-armed — its 07-09
  "cosmetic" narrowing predates the expansion; the custom-kind
  precondition it waited on is proven (0036, base-harness example).

## Parked (pointers only)

- Sweep-dock blend (John, 07-20): flume-dock base uplifted to
  temper-standard discipline + sweep preset shipped (flume-dock 748c487,
  688a16d, e499e18; SPEC decisions 15-16, build items 10-11); testbed
  centercode-platform bd75aeddc4 lands: remedy/governs grammar,
  engineering postures pack, sweep-discipline rule (emit+check green, 51
  members). Design ledger = this session's grill (A-F + amendments).
  Calibration waves over 9c8d78aa47..d0734c2643 in flight (wave A sonnet
  on dock/cal-wave-a; wave B haiku next, tagged).
  Watch items: standard-delta backfill (park, don't drown);
  install.gate-installed advisory in testbed check.

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
- Docs-language candidates (when docs are written): the determinism
  ladder — "push every check to the most deterministic layer that can
  express it"; the harness pin (John, 07-18) — "our job in the harness
  is to name the invariants, and let the loop settle".
- Consumer-format constants' home (parked, John 07-20): `MAX_IMPORT_HOPS`
  (src/graph.rs) is a target-format fact baked as an engine constant with
  its cite in a comment — it drifted (5 vs the real 4, fixed 14719f2).
  Considered declaring such caps on the cited kind (memory kind's
  import-directive), engine reading them through the lock; deferred — a
  lock-schema extension for a single one-format fact, and it only dedups
  the low-harm internal axis (guidance prose is advisory; the engine
  constant is the sole load-bearing home). **Trigger to revisit: a second
  import-bearing format** makes per-format declared caps non-speculative.
  Freshness (engine-vs-reality drift) is an inherent bound — temper is
  offline/decidable-only, so only re-verification catches it; not encoded.
- Base harness dogfood: primer `docs/base-harness-primer.md`; example at
  `examples/base-harness/` (third cut shipped 549969f); built-in-kind doc
  audit at `docs/market-formats.md`. Sequencing: stranger dry run next,
  then channel 3.
- On John:
  **Rotate NPM_TOKEN** — the current one was pasted in chat (07-19,
  v0.0.8 rescue); treat as exposed. New token → `gh secret set
  NPM_TOKEN` + `.env`. Note the expiry cadence: the prior token died
  in ~14 days and cost a release-day debug.
  Apple Developer notarizing (decide at release); USPTO name screen;
  CHANGELOG for the shipped 0.0.x npm cuts.

## Standing discipline (mechanical, paid for)

- Wake-then-loop as its own background task; `git status` before any
  restore; never edit tracked files while a tick runs (plan stages -A).
- At session open: sweep `.flume/friction/`, `.flume/refactor/`, and
  `.flume/amendments/` (ratify or decline, then delete — 0044);
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
